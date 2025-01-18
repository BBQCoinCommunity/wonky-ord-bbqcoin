use super::num::Num;
use once_cell::sync::Lazy;

pub const PROTOCOL_LITERAL: &str = "bqc-20";

pub static MAXIMUM_SUPPLY: Lazy<Num> = Lazy::new(|| Num::from(u64::MAX));
