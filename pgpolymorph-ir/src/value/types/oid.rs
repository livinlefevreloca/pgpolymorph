#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PgOid {
    pub value: u32,
}

impl PgOid {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl From<u32> for PgOid {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}
