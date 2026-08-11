#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgMoney {
    pub amount: i64,
}

impl PgMoney {
    pub const fn new(amount: i64) -> Self {
        Self { amount }
    }
}

impl From<i64> for PgMoney {
    fn from(amount: i64) -> Self {
        Self::new(amount)
    }
}
