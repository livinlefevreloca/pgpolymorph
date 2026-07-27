#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PgUuid {
    pub bytes: [u8; 16],
}

impl PgUuid {
    pub const fn new(bytes: [u8; 16]) -> Self {
        Self { bytes }
    }
}
