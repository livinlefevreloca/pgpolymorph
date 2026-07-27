#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PgBool {
    pub value: bool,
}

impl PgBool {
    pub const fn new(value: bool) -> Self {
        Self { value }
    }
}

impl From<bool> for PgBool {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}
