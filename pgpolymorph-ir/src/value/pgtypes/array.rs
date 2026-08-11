use crate::schema::PgType;
use crate::value::PgValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayDimension {
    pub length: i32,
    /// PostgreSQL array dimension lower bound from the binary header.
    ///
    /// Each dimension is encoded as `(length, lower_bound)`. SQL arrays commonly use
    /// lower bound 1, so `ARRAY[10, 20, 30]` is indexed `[1:3]` rather than `[0:2]`.
    /// Slices and some constructs can produce other bounds; this crate preserves the
    /// binary header value verbatim and does not re-index elements to 0-based.
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
