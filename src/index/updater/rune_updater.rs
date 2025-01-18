use {
  super::*,
  crate::runes::{varint, Edict, Runestone},
};
use crate::runes::CLAIM_BIT;

fn claim(id: u128) -> Option<u128> {
  (id & CLAIM_BIT != 0).then_some(id ^ CLAIM_BIT)
}

struct Allocation {
  balance: u128,
  divisibility: u8,
  id: u128,
  mint: Option<Terms>,
  rune: Rune,
  premine: u128,
  spacers: u32,
  symbol: Option<char>,
  turbo: bool,
}

pub(super) struct RuneUpdater<'a, 'db, 'tx> {
  height: u32,
  id_to_entry: &'a mut Table<'db, 'tx, RuneIdValue, RuneEntryValue>,
  inscription_id_to_inscription_entry:
  &'a Table<'db, 'tx, &'static InscriptionIdValue, InscriptionEntryValue>,
  inscription_id_to_rune: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, u128>,
  minimum: Rune,
  outpoint_to_balances: &'a mut Table<'db, 'tx, &'static OutPointValue, &'static [u8]>,
  rune_to_id: &'a mut Table<'db, 'tx, u128, RuneIdValue>,
  runes: u64,
  statistic_to_count: &'a mut Table<'db, 'tx, u64, u64>,
  timestamp: u32,
}

impl<'a, 'db, 'tx> RuneUpdater<'a, 'db, 'tx> {
  pub(super) fn new(
    height: u32,
    outpoint_to_balances: &'a mut Table<'db, 'tx, &'static OutPointValue, &'static [u8]>,
    id_to_entry: &'a mut Table<'db, 'tx, RuneIdValue, RuneEntryValue>,
    inscription_id_to_inscription_entry: &'a Table<
      'db,
      'tx,
      &'static InscriptionIdValue,
      InscriptionEntryValue,
    >,
    inscription_id_to_rune: &'a mut Table<'db, 'tx, &'static InscriptionIdValue, u128>,
    rune_to_id: &'a mut Table<'db, 'tx, u128, RuneIdValue>,
    statistic_to_count: &'a mut Table<'db, 'tx, u64, u64>,
    timestamp: u32,
    minimum: Rune,
  ) -> Result<Self> {
    let runes = statistic_to_count
        .get(&Statistic::Runes.into())?
        .map(|x| x.value())
        .unwrap_or(0);
    Ok(Self {
      height,
      id_to_entry,
      minimum,
      outpoint_to_balances,
      inscription_id_to_inscription_entry,
      inscription_id_to_rune,
      rune_to_id,
      runes,
      statistic_to_count,
      timestamp,
    })
  }

  pub(super) fn index_runes(&mut self, index: usize, tx: &Transaction, txid: Txid) -> Result<()> {
    let runestone = Runestone::from_transaction(tx);

    // A mapping of rune ID to un-allocated balance of that rune
    let mut unallocated: HashMap<u128, u128> = HashMap::new();

    // Increment unallocated runes with the runes in this transaction's inputs
    for input in &tx.input {
      if let Some(guard) = self
        .outpoint_to_balances
        .remove(&input.previous_output.store())?
      {
        let buffer = guard.value();
        let mut i = 0;
        while i < buffer.len() {
          let (id, len) = varint::decode(&buffer[i..]);
          i += len;
          let (balance, len) = varint::decode(&buffer[i..]);
          i += len;
          *unallocated.entry(id).or_default() += balance;
        }
      }
    }

    let cenotaph = runestone
        .as_ref()
        .map(|runestone| runestone.cenotaph)
        .unwrap_or_default();

    let default_output = runestone.as_ref().and_then(|runestone| {
      runestone
          .pointer
          .and_then(|default| usize::try_from(default).ok())
    });

    // A vector of allocated transaction output rune balances
    let mut allocated: Vec<HashMap<u128, u128>> = vec![HashMap::new(); tx.output.len()];

    if let Some(runestone) = runestone {
      // Determine if this runestone contains a valid issuance
      let mut allocation = match runestone.etching {
        Some(etching) => {
          if etching
              .rune
              .map(|rune| rune < self.minimum || rune.is_reserved())
              .unwrap_or_default()
              || etching
              .rune
              .and_then(|rune| self.rune_to_id.get(rune.0).transpose())
              .transpose()?
              .is_some()
          {
            None
          } else {
            let rune = if let Some(rune) = etching.rune {
              rune
            } else {
              let reserved_runes = self
                  .statistic_to_count
                  .get(&Statistic::ReservedRunes.into())?
                  .map(|entry| entry.value())
                  .unwrap_or_default();

              self
                  .statistic_to_count
                  .insert(&Statistic::ReservedRunes.into(), reserved_runes + 1)?;

              Rune::reserved(reserved_runes.into())
            };

            // Construct an allocation, representing the new runes that may be
            // allocated. Beware: Because it would require constructing a block
            // with 2**16 + 1 transactions, there is no test that checks that
            // an eching in a transaction with an out-of-bounds index is
            // ignored.
            match u16::try_from(index) {
              Ok(index) => Some(Allocation {
                balance: u128::MAX,
                divisibility: etching.divisibility.unwrap_or_default(),
                id: u128::from(self.height) << 16 | u128::from(index),
                rune,
                premine: etching.premine.unwrap_or_default(),
                spacers: etching.spacers.unwrap_or_default(),
                symbol: etching.symbol,
                mint: etching.terms.map(|mint| Terms {
                  cap: mint.cap,
                  limit: mint.limit.map(|limit| limit.clamp(0, runes::MAX_LIMIT)),
                  height: mint.height,
                  offset: mint.offset,
                }),
                turbo: etching.turbo,
              }),
              Err(_) => {
                None
              },
            }
          }
        }
        None => {
          None
        },
      };

      let mut premine_amount = 0;

      if !cenotaph {
        let mut mintable: HashMap<u128, u128> = HashMap::new();

        let mut claims = runestone
            .edicts
            .iter()
            .filter_map(|edict| claim(edict.id))
            .collect::<Vec<u128>>();
        claims.sort();
        claims.dedup();
        for id in claims {
          if let Ok(key) = RuneId::try_from(id) {
            if let Some(entry) = self.id_to_entry.get(&key.store())? {
              let entry = RuneEntry::load(entry.value());
              let Ok(limit) = entry.mintable(self.height.into()) else {
                continue;
              };
              mintable.insert(id, limit);
            }
          }
        }

        let limits = mintable.clone();

        for Edict { id, amount, output } in runestone.edicts {
          let Ok(output) = usize::try_from(output) else {
            continue;
          };

          // Skip edicts not referring to valid outputs
          if output >= tx.output.len() {
            continue;
          }

          let (balance, id) = if id == 0 {
            // If this edict allocates new issuance runes, skip it
            // if no issuance was present, or if the issuance was invalid.
            // Additionally, replace ID 0 with the newly assigned ID, and
            // get the unallocated balance of the issuance.
            match allocation.as_mut() {
              Some(Allocation { balance, id, .. }) => {
                premine_amount = premine_amount + amount;
                (balance, *id)
              },
              None => continue,
            }
          } else if let Some(claim) = claim(id) {
            match mintable.get_mut(&claim) {
              Some(balance) => (balance, claim),
              None => continue,
            }
          } else {
            // Get the unallocated balance of the given ID
            match unallocated.get_mut(&id) {
              Some(balance) => (balance, id),
              None => continue,
            }
          };

          let mut allocate = |balance: &mut u128, amount: u128, output: usize| {
            if amount > 0 {
              *balance -= amount;
              *allocated[output].entry(id).or_default() += amount;
            }
          };

          if output == tx.output.len() {
            // find non-OP_RETURN outputs
            let destinations = tx
                .output
                .iter()
                .enumerate()
                .filter_map(|(output, tx_out)| {
                  (!tx_out.script_pubkey.is_op_return()).then_some(output)
                })
                .collect::<Vec<usize>>();

            if amount == 0 {
              // if amount is zero, divide balance between eligible outputs
              let amount = *balance / destinations.len() as u128;
              let remainder = usize::try_from(*balance % destinations.len() as u128).unwrap();

              for (i, output) in destinations.iter().enumerate() {
                allocate(
                  balance,
                  if i < remainder { amount + 1 } else { amount },
                  *output,
                );
              }
            } else {
              // if amount is non-zero, distribute amount to eligible outputs
              for output in destinations {
                allocate(balance, amount.min(*balance), output);
              }
            }
          } else {
            // Get the allocatable amount
            let amount = if amount == 0 {
              *balance
            } else {
              amount.min(*balance)
            };

            allocate(balance, amount, output);
          }
        }

        // increment entries with minted runes
        for (id, amount) in mintable {
          let minted = limits[&id] - amount;
          if minted > 0 {
            let id = RuneId::try_from(id).unwrap().store();
            let mut entry = RuneEntry::load(self.id_to_entry.get(id)?.unwrap().value());
            entry.supply += minted;
            entry.mints += 1;
            self.id_to_entry.insert(id, entry.store())?;
          }
        }
      }

      if let Some(Allocation {
        balance,
        divisibility,
        id,
        mint,
        rune,
        premine,
        spacers,
        symbol,
        turbo,
      }) = allocation
      {
        let id = RuneId::try_from(id).unwrap();
        self.rune_to_id.insert(rune.0, id.store())?;
        let number = self.runes;
        self.runes += 1;

        self
            .statistic_to_count
            .insert(&Statistic::Runes.into(), self.runes)?;

        self.id_to_entry.insert(
          id.store(),
          RuneEntry {
            block: self.height.into(),
            burned: 0,
            divisibility,
            etching: txid,
            mints: 0,
            number,
            premine: premine_amount,
            terms: mint.and_then(|mint| (!cenotaph).then_some(mint)),
            rune,
            spacers,
            supply: u128::MAX - balance - premine_amount,
            symbol,
            timestamp: self.timestamp.into(),
            turbo,
          }
              .store(),
        )?;

        let inscription_id = InscriptionId { txid, index: 0 };

        if self
            .inscription_id_to_inscription_entry
            .get(&inscription_id.store())?
            .is_some()
        {
          self
              .inscription_id_to_rune
              .insert(&inscription_id.store(), rune.0)?;
        }
      }
    }

    let mut burned: HashMap<u128, u128> = HashMap::new();

    if cenotaph {
      for (id, balance) in unallocated {
        *burned.entry(id).or_default() += balance;
      }
    } else {
      // assign all un-allocated runes to the default output, or the first non
      // OP_RETURN output if there is no default, or if the default output is
      // too large
      if let Some(vout) = default_output
          .filter(|vout| *vout < allocated.len())
          .or_else(|| {
            tx.output
                .iter()
                .enumerate()
                .find(|(_vout, tx_out)| !tx_out.script_pubkey.is_op_return())
                .map(|(vout, _tx_out)| vout)
          })
      {
        for (id, balance) in unallocated {
          if balance > 0 {
            *allocated[vout].entry(id).or_default() += balance;
          }
        }
      } else {
        for (id, balance) in unallocated {
          if balance > 0 {
            *burned.entry(id).or_default() += balance;
          }
        }
      }
    }

    // update outpoint balances
    let mut buffer: Vec<u8> = Vec::new();
    for (vout, balances) in allocated.into_iter().enumerate() {
      if balances.is_empty() {
        continue;
      }

      // increment burned balances
      if tx.output[vout].script_pubkey.is_op_return() {
        for (id, balance) in &balances {
          *burned.entry(*id).or_default() += balance;
        }
        continue;
      }

      buffer.clear();

      let mut balances = balances.into_iter().collect::<Vec<(u128, u128)>>();

      // Sort balances by id so tests can assert balances in a fixed order
      balances.sort();

      for (id, balance) in balances {
        varint::encode_to_vec(id, &mut buffer);
        varint::encode_to_vec(balance, &mut buffer);
      }

      self.outpoint_to_balances.insert(
        &OutPoint {
          txid,
          vout: vout.try_into().unwrap(),
        }
        .store(),
        buffer.as_slice(),
      )?;
    }

    // increment entries with burned runes
    for (id, amount) in burned {
      let id = RuneId::try_from(id).unwrap().store();
      let mut entry = RuneEntry::load(self.id_to_entry.get(id)?.unwrap().value());
      entry.burned += amount;
      self.id_to_entry.insert(id, entry.store())?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn claim_from_id() {
    assert_eq!(claim(1), None);
    assert_eq!(claim(1 | CLAIM_BIT), Some(1));
  }
}
