//! COPY binary file header.

use crate::binary::be;
use crate::binary::constants::{self, COPY_MAGIC};
use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PgBinaryHeader {
    pub flags: i32,
    pub extension: Vec<u8>,
}

impl PgBinaryHeader {
    pub fn parse(data: &[u8]) -> Result<(Self, usize)> {
        validate_magic(data)?;
        let rest = &data[COPY_MAGIC.len()..];

        let (flags, rest) = read_be_i32(rest)?;
        let (ext_len, rest) = read_be_i32(rest)?;
        if ext_len < 0 {
            return Err(Error::InvalidHeader {
                reason: "negative extension length",
            });
        }
        let ext_len = ext_len as usize;
        if rest.len() < ext_len {
            return Err(Error::UnexpectedEof {
                expected: ext_len,
                available: rest.len(),
            });
        }
        let (extension, _) = rest.split_at(ext_len);
        let consumed = COPY_MAGIC.len() + constants::I32_BYTES + constants::I32_BYTES + ext_len;
        Ok((
            PgBinaryHeader {
                flags,
                extension: extension.to_vec(),
            },
            consumed,
        ))
    }

    pub fn write_to(buf: &mut Vec<u8>) {
        buf.extend_from_slice(COPY_MAGIC);
        buf.extend_from_slice(&0i32.to_be_bytes());
        buf.extend_from_slice(&0i32.to_be_bytes());
    }
}

fn validate_magic(data: &[u8]) -> Result<()> {
    if data.is_empty() {
        return Err(Error::UnexpectedEof {
            expected: COPY_MAGIC.len(),
            available: 0,
        });
    }
    if data.len() < COPY_MAGIC.len() {
        return Err(Error::InvalidMagic);
    }
    if &data[..COPY_MAGIC.len()] != COPY_MAGIC.as_slice() {
        return Err(Error::InvalidMagic);
    }
    Ok(())
}

fn read_be_i32(data: &[u8]) -> Result<(i32, &[u8])> {
    if data.len() < constants::I32_BYTES {
        return Err(Error::UnexpectedEof {
            expected: constants::I32_BYTES,
            available: data.len(),
        });
    }
    let (bytes, rest) = data.split_at(constants::I32_BYTES);
    Ok((
        be::read_be_i32(bytes).expect("exact length"),
        rest,
    ))
}
