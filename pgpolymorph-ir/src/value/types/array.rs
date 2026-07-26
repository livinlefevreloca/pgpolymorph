use crate::schema::PgType;
use crate::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArrayDimension {
    pub length: i32,
    pub lower_bound: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PgArray {
    pub element_type: PgType,
    pub dimensions: Vec<ArrayDimension>,
    pub elements: Vec<Value>,
}

impl PgArray {
    pub fn new(
        element_type: PgType,
        dimensions: Vec<ArrayDimension>,
        elements: Vec<Value>,
    ) -> Self {
        Self {
            element_type,
            dimensions,
            elements,
        }
    }
}
