#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
