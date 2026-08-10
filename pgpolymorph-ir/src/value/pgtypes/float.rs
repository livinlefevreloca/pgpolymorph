#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgFloat4 {
    pub value: f32,
}

impl PgFloat4 {
    pub const fn new(value: f32) -> Self {
        Self { value }
    }
}

impl From<f32> for PgFloat4 {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgFloat8 {
    pub value: f64,
}

impl PgFloat8 {
    pub const fn new(value: f64) -> Self {
        Self { value }
    }
}

impl From<f64> for PgFloat8 {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}
