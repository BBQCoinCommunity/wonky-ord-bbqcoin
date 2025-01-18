use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub runes: BTreeMap<Rune, RuneInfo>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RuneInfo {
  pub block: u64,
  pub burned: u128,
  pub divisibility: u8,
  pub etching: Txid,
  pub height: u64,
  pub id: RuneId,
  pub index: u32,
  pub terms: Option<Terms>,
  pub mints: u128,
  pub number: u64,
  pub premine: u128,
  pub rune: Rune,
  pub spacers: u32,
  pub supply: u128,
  pub symbol: Option<char>,
  pub timestamp: DateTime<Utc>,
  pub turbo: bool,
  pub tx: u32,
}

pub(crate) fn run(options: Options) -> SubcommandResult {
  let index = Index::open(&options)?;

  ensure!(
    index.has_rune_index(),
    "`ord runes` requires index created with `--index-runes` flag",
  );

  index.update()?;

  Ok(Box::new(Output {
    runes: index
      .runes()?
      .into_iter()
      .map(
        |(
          id,
          entry @ RuneEntry {
            block,
            burned,
            divisibility,
            etching,
            terms,
            mints,
            number,
            premine,
            rune,
            spacers,
            supply,
            symbol,
            timestamp,
            turbo,
          },
        )| {
          (
            rune,
            RuneInfo {
              block,
              burned,
              divisibility,
              etching,
              height: id.height,
              id,
              index: id.index,
              terms,
              mints,
              number,
              premine,
              timestamp: crate::timestamp(timestamp),
              rune,
              spacers,
              supply,
              symbol,
              turbo,
              tx: id.index,
            },
          )
        },
      )
      .collect::<BTreeMap<Rune, RuneInfo>>(),
  }))
}
