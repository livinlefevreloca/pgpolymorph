#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgBytea {
    pub bytes: Vec<u8>,
}

impl PgBytea {
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }
}

impl From<Vec<u8>> for PgBytea {
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(bytes)
    }
}

impl AsRef<[u8]> for PgBytea {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}
