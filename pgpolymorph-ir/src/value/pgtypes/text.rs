#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PgText {
    pub value: String,
}

impl PgText {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<String> for PgText {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for PgText {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
