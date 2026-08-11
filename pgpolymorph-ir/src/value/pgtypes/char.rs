/// PostgreSQL `char` (internal single-character type, not `varchar`).
///
/// Stored as `i16` because PostgreSQL binary format sends the internal `char` type
/// as a 2-byte big-endian integer, not as a Rust `char` or UTF-8 code point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
