/// PostgreSQL system identifier type (`name`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgName {
    pub value: String,
}

impl PgName {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<String> for PgName {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
