//! COPY binary file header.

use crate::binary::constants::COPY_MAGIC;
use crate::binary::field::FieldReader;
use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CopyHeader {
    pub flags: i32,
    pub extension: Vec<u8>,
}

impl CopyHeader {
    pub fn parse(data: &[u8]) -> Result<(Self, usize)> {
        if data.len() < COPY_MAGIC.len() {
            return Err(if data.is_empty() {
                Error::UnexpectedEof {
                    expected: COPY_MAGIC.len(),
                    available: 0,
                }
            } else {
                Error::InvalidMagic
            });
        }
        if &data[..COPY_MAGIC.len()] != COPY_MAGIC.as_slice() {
            return Err(Error::InvalidMagic);
        }

        let mut reader = FieldReader::new(&data[COPY_MAGIC.len()..]);
        let flags = reader.read_i32()?;
        let ext_len = reader.read_i32()?;
        if ext_len < 0 {
            return Err(Error::UnexpectedEof {
                expected: ext_len as usize,
                available: reader.remaining(),
            });
        }
        let extension = reader.read_bytes(ext_len as usize)?.to_vec();
        let consumed = COPY_MAGIC.len() + reader.consumed();
        Ok((
            CopyHeader { flags, extension },
            consumed,
        ))
    }

    pub fn write_to(buf: &mut Vec<u8>) {
        buf.extend_from_slice(COPY_MAGIC);
        buf.extend_from_slice(&0i32.to_be_bytes());
        buf.extend_from_slice(&0i32.to_be_bytes());
    }
}
