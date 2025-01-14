use super::*;
use bitcoin::blockdata::constants::COIN_VALUE;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Display, PartialOrd)]
pub(crate) struct Epoch(pub(crate) u32);

impl Epoch {
  pub(crate) const STARTING_SATS: [Sat; 32] = [
    Sat(0 * COIN_VALUE as u64),
    Sat(49084400000 * COIN_VALUE as u64),
    Sat(49084500000 * COIN_VALUE as u64),
    Sat(240000000000 * COIN_VALUE as u64),
    Sat(480000000000 * COIN_VALUE as u64),
    Sat(720000000000 * COIN_VALUE as u64),
    Sat(960000000000 * COIN_VALUE as u64),
    Sat(1200000000000 * COIN_VALUE as u64),
    Sat(1440000000000 * COIN_VALUE as u64),
    Sat(1680000000000 * COIN_VALUE as u64),
    Sat(1920000000000 * COIN_VALUE as u64),
    Sat(2160000000000 * COIN_VALUE as u64),
    Sat(2400000000000 * COIN_VALUE as u64),
    Sat(2640000000000 * COIN_VALUE as u64),
    Sat(2880000000000 * COIN_VALUE as u64),
    Sat(3120000000000 * COIN_VALUE as u64),
    Sat(3360000000000 * COIN_VALUE as u64),
    Sat(3600000000000 * COIN_VALUE as u64),
    Sat(3840000000000 * COIN_VALUE as u64),
    Sat(4080000000000 * COIN_VALUE as u64),
    Sat(4320000000000 * COIN_VALUE as u64),
    Sat(4560000000000 * COIN_VALUE as u64),
    Sat(4800000000000 * COIN_VALUE as u64),
    Sat(5040000000000 * COIN_VALUE as u64),
    Sat(5280000000000 * COIN_VALUE as u64),
    Sat(5520000000000 * COIN_VALUE as u64),
    Sat(5760000000000 * COIN_VALUE as u64),
    Sat(6000000000000 * COIN_VALUE as u64),
    Sat(6240000000000 * COIN_VALUE as u64),
    Sat(6480000000000 * COIN_VALUE as u64),
    Sat(6720000000000 * COIN_VALUE as u64),
    Sat(6960000000000 * COIN_VALUE as u64),
    Sat(7200000000000 * COIN_VALUE as u64),
    Sat(7440000000000 * COIN_VALUE as u64),
    Sat(7680000000000 * COIN_VALUE as u64),
  ];

  pub(crate) fn subsidy(self) -> u64 {
    match self.0 {
        0 => 42 * COIN_VALUE,
        1 => 351_582 * COIN_VALUE,
        2 => 21 * COIN_VALUE,
        3 => 10.5 * COIN_VALUE,
        4 => 5.25 * COIN_VALUE,
        5 => 2.625 * COIN_VALUE,
        6 => 1.3125 * COIN_VALUE,
        7 => 0.65625 * COIN_VALUE,
        8 => 0.328125 * COIN_VALUE,
        9 => 0.1640625 * COIN_VALUE,
        10 => 0.08203125 * COIN_VALUE,
        11 => 0.04101563 * COIN_VALUE,
        12 => 0.02050781 * COIN_VALUE,
        13 => 0.01025391 * COIN_VALUE,
        14 => 0.00512695 * COIN_VALUE,
        15 => 0.00256348 * COIN_VALUE,
        16 => 0.00128174 * COIN_VALUE,
        17 => 0.00064087 * COIN_VALUE,
        18 => 0.00032043 * COIN_VALUE,
        19 => 0.00016022 * COIN_VALUE,
        20 => 0.00008011 * COIN_VALUE,
        21 => 0.0000400543 * COIN_VALUE,
        22 => 0.0000200272 * COIN_VALUE,
        23 => 0.0000100136 * COIN_VALUE,
        24 => 0.00000500679 * COIN_VALUE,
        25 => 0.0000025034 * COIN_VALUE,
        26 => 0.0000012517 * COIN_VALUE,
        27 => 0.000000625849 * COIN_VALUE,
        28 => 0.000000312924 * COIN_VALUE,
        29 => 0.000000156462 * COIN_VALUE,
        30 => 0.0000000782311 * COIN_VALUE,
        31 => 0.0000000391155 * COIN_VALUE,
        32 => 0.0000000195578 * COIN_VALUE,
        _ => panic!("bad epoch"),
    }
}

  pub(crate) fn starting_sat(self) -> Sat {
    *Self::STARTING_SATS
      .get(usize::try_from(self.0).unwrap())
      .unwrap_or_else(|| Self::STARTING_SATS.last().unwrap())
  }

  pub(crate) fn starting_height(self) -> Height {
    match self.0 {
        0 => Height(0),
        1 => Height(490_844),
        2 => Height(490_845),
        3 => Height(2_400_000),
        4 => Height(4_800_000),
        5 => Height(7_200_000),
        6 => Height(9_600_000),
        7 => Height(12_000_000),
        8 => Height(14_400_000),
        9 => Height(16_800_000),
        10 => Height(19_200_000),
        11 => Height(21_600_000),
        12 => Height(24_000_000),
        13 => Height(26_400_000),
        14 => Height(28_800_000),
        15 => Height(31_200_000),
        16 => Height(33_600_000),
        17 => Height(36_000_000),
        18 => Height(38_400_000),
        19 => Height(40_800_000),
        20 => Height(43_200_000),
        21 => Height(45_600_000),
        22 => Height(48_000_000),
        23 => Height(50_400_000),
        24 => Height(52_800_000),
        25 => Height(55_200_000),
        26 => Height(57_600_000),
        27 => Height(60_000_000),
        28 => Height(62_400_000),
        29 => Height(64_800_000),
        30 => Height(67_200_000),
        31 => Height(69_600_000),
        32 => Height(72_000_000),
        33 => Height(74_400_000),
        34 => Height(76_800_000),
        _ => panic!("bad epoch"),
    }
  }
}

impl PartialEq<u32> for Epoch {
  fn eq(&self, other: &u32) -> bool {
    self.0 == *other
  }
}

impl From<Sat> for Epoch {
  fn from(sat: Sat) -> Self {
    for (epoch, &starting_sat) in Self::STARTING_SATS.iter().enumerate() {
        if sat < starting_sat {
            return Epoch(epoch as u32 - 1);
        }
    }
    Epoch((Self::STARTING_SATS.len() - 1) as u32)
  }
}

impl From<Height> for Epoch {
  fn from(height: Height) -> Self {
    let epochs = [0, 490_844, 490_845, 2_400_000, 4_800_000, 7_200_000, 9_600_000, 12_000_000, 14_400_000, 16_800_000, 19_200_000, 21_600_000, 24_000_000, 26_400_000, 28_800_000, 31_200_000, 33_600_000, 36_000_000, 38_400_000, 40_800_000, 43_200_000, 45_600_000, 48_000_000, 50_400_000, 52_800_000, 55_200_000, 57_600_000, 60_000_000, 62_400_000, 64_800_000, 67_200_000, 69_600_000, 72_000_000, 74_400_000, 76_800_000];
    for (epoch, &starting_height) in epochs.iter().enumerate() {
        if height.0 < starting_height {
            return Epoch(epoch as u32 - 1);
        }
    }
    Epoch((epochs.len() - 1) as u32)
  }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn starting_sat() {
        assert_eq!(Epoch(0).starting_sat(), Sat(0));
    }

    #[test]
    fn subsidy() {
        assert_eq!(Epoch(0).subsidy(), 42 * COIN_VALUE);
        assert_eq!(Epoch(1).subsidy(), 351_582 * COIN_VALUE);
        assert_eq!(Epoch(2).subsidy(), 21 * COIN_VALUE);
    }

    #[test]
    fn starting_height() {
        assert_eq!(Epoch(0).starting_height(), Height(0));
        assert_eq!(Epoch(1).starting_height(), Height(490_844));
    }

    #[test]
    fn from_height() {
        assert_eq!(Epoch::from(Height(0)), Epoch(0));
        assert_eq!(Epoch::from(Height(490_844)), Epoch(1));
    }

    #[test]
    fn from_sat() {
        for (epoch, starting_sat) in Epoch::STARTING_SATS.iter().enumerate() {
            if epoch > 0 {
                assert_eq!(
                    Epoch::from(Sat(starting_sat.n() - 1)),
                    Epoch((epoch - 1) as u64)
                );
            }
            assert_eq!(Epoch::from(*starting_sat), Epoch(epoch as u64));
            assert_eq!(Epoch::from(*starting_sat + 1), Epoch(epoch as u64));
        }
        assert_eq!(Epoch::from(Sat(0)), Epoch(0));
        assert_eq!(Epoch::from(Sat(1)), Epoch(0));
        assert_eq!(Epoch::from(Epoch(1).starting_sat()), Epoch(1));
        assert_eq!(Epoch::from(Epoch(1).starting_sat() + 1), Epoch(1));
    }

    #[test]
    fn eq() {
        assert_eq!(Epoch(0), 0);
        assert_eq!(Epoch(100), 100);
    }
}
