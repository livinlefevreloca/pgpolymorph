use crate::schema::PgType;
use crate::value::PgValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayDimension {
    pub length: i32,
    /// PostgreSQL array dimension lower bound from the binary header (often `1` for SQL
    /// arrays, but not always).
    pub lower_bound: i32,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgArray {
    pub element_type: PgType,
    pub dimensions: Vec<ArrayDimension>,
    pub elements: Vec<PgValue>,
}

impl PgArray {
    pub fn new(
        element_type: PgType,
        dimensions: Vec<ArrayDimension>,
        elements: Vec<PgValue>,
    ) -> Self {
        Self {
            element_type,
            dimensions,
            elements,
        }
    }
}
