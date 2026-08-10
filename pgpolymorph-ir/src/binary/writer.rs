//! Incremental COPY binary blob writer.

use crate::binary::constants::{self, FOOTER_SENTINEL};
use crate::binary::header::PgBinaryHeader;

pub(crate) struct PgBinaryWriter {
    buf: Vec<u8>,
}

impl PgBinaryWriter {
    pub fn new() -> Self {
        let mut buf = Vec::new();
        PgBinaryHeader::write_to(&mut buf);
        Self { buf }
    }

    pub fn finish(self) -> Vec<u8> {
        self.buf
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    pub fn write_field(
        &mut self,
        field: EncodedField,
    ) -> crate::error::Result<()> {
        match field {
            EncodedField::Null => {
                self.write_bytes(&constants::COPY_FIELD_NULL.to_be_bytes());
            }
            EncodedField::NonNull(payload) => {
                let len = i32::try_from(payload.len()).map_err(|_| {
                    crate::error::Error::FieldTooLarge {
                        len: payload.len() as i64,
                    }
                })?;
                self.write_bytes(&len.to_be_bytes());
                debug_assert_eq!(constants::COPY_FIELD_LEN_BYTES, len.to_be_bytes().len());
                self.write_bytes(&payload);
            }
        }
        Ok(())
    }

    pub fn write_tuple(
        &mut self,
        field_count: i16,
        fields: impl IntoIterator<Item = EncodedField>,
    ) -> crate::error::Result<()> {
        self.write_bytes(&field_count.to_be_bytes());
        for field in fields {
            self.write_field(field)?;
        }
        Ok(())
    }

    pub fn write_footer(&mut self) {
        self.write_bytes(&FOOTER_SENTINEL.to_be_bytes());
    }
}

impl Default for PgBinaryWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EncodedField {
    Null,
    NonNull(Vec<u8>),
}
