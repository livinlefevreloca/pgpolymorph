#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PgInterval {
    pub micros: i64,
    pub days: i32,
    pub months: i32,
}

impl PgInterval {
    pub const fn new(micros: i64, days: i32, months: i32) -> Self {
        Self {
            micros,
            days,
            months,
        }
    }
}
