/// Fixed-scale currency amount in micro-dollars (PostgreSQL `money` wire encoding).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PgMoney {
    pub amount: i64,
}

impl PgMoney {
    pub const fn new(amount: i64) -> Self {
        Self { amount }
    }
}
