/// Variable-length character string (`varchar` / `character varying`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgVarchar {
    pub value: String,
    /// Maximum character length from the column typmod (`varchar(n)`), or `None` if unlimited.
    pub max_len: Option<u32>,
}

impl PgVarchar {
    pub fn new(value: impl Into<String>, max_len: Option<u32>) -> Self {
        Self {
            value: value.into(),
            max_len,
        }
    }
}

impl From<String> for PgVarchar {
    fn from(value: String) -> Self {
        Self::new(value, None)
    }
}

impl From<&str> for PgVarchar {
    fn from(value: &str) -> Self {
        Self::new(value, None)
    }
}
