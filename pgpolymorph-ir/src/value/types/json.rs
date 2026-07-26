/// UTF-8 JSON text (`json` type — not pre-parsed).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PgJson {
    pub text: String,
}

impl PgJson {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

/// Binary JSON (`jsonb` type) with wire version byte stripped at the IR boundary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PgJsonb {
    pub version: u8,
    pub json: String,
}

impl PgJsonb {
    pub fn new(version: u8, json: impl Into<String>) -> Self {
        Self {
            version,
            json: json.into(),
        }
    }
}
