//! Incremental COPY binary blob writer.

use crate::binary::constants::FOOTER_SENTINEL;
use crate::binary::header::CopyHeader;
use crate::binary::wire;

pub(crate) struct CopyWriter {
    buf: Vec<u8>,
}

impl CopyWriter {
    pub fn new() -> Self {
        let mut buf = Vec::new();
        CopyHeader::write_to(&mut buf);
        Self { buf }
    }

    pub fn finish(self) -> Vec<u8> {
        self.buf
    }

    pub fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.buf
    }
}

impl Default for CopyWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EncodedField {
    Null,
    NonNull(Vec<u8>),
}

pub(crate) fn write_tuple(
    writer: &mut CopyWriter,
    field_count: i16,
    fields: impl IntoIterator<Item = EncodedField>,
) -> crate::error::Result<()> {
    writer
        .as_mut_vec()
        .extend_from_slice(&field_count.to_be_bytes());
    for field in fields {
        match field {
            EncodedField::Null => {
                writer
                    .as_mut_vec()
                    .extend_from_slice(&wire::COPY_FIELD_NULL.to_be_bytes());
            }
            EncodedField::NonNull(payload) => {
                let len = i32::try_from(payload.len()).map_err(|_| {
                    crate::error::Error::FieldTooLarge {
                        len: payload.len() as i64,
                    }
                })?;
                writer.as_mut_vec().extend_from_slice(&len.to_be_bytes());
                writer.as_mut_vec().extend_from_slice(&payload);
            }
        }
    }
    Ok(())
}

pub(crate) fn write_footer(writer: &mut CopyWriter) {
    writer
        .as_mut_vec()
        .extend_from_slice(&FOOTER_SENTINEL.to_be_bytes());
}
