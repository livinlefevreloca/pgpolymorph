/// PostgreSQL `char` (internal single-character type, not `varchar`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PgChar {
    pub value: i16,
}

impl PgChar {
    pub const fn new(value: i16) -> Self {
        Self { value }
    }
}

impl From<i16> for PgChar {
    fn from(value: i16) -> Self {
        Self::new(value)
    }
}
