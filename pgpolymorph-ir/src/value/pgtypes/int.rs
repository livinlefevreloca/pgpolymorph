#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgInt2 {
    pub value: i16,
}

impl PgInt2 {
    pub const fn new(value: i16) -> Self {
        Self { value }
    }
}

impl From<i16> for PgInt2 {
    fn from(value: i16) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgInt4 {
    pub value: i32,
}

impl PgInt4 {
    pub const fn new(value: i32) -> Self {
        Self { value }
    }
}

impl From<i32> for PgInt4 {
    fn from(value: i32) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgInt8 {
    pub value: i64,
}

impl PgInt8 {
    pub const fn new(value: i64) -> Self {
        Self { value }
    }
}

impl From<i64> for PgInt8 {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}
