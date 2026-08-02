//! COPY binary file header.

use crate::binary::buffer_view::BufferView;
use crate::binary::constants::COPY_MAGIC;
use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PgBinaryHeader {
    pub flags: i32,
    pub extension: Vec<u8>,
}

impl PgBinaryHeader {
    pub fn parse(data: &[u8]) -> Result<(Self, usize)> {
        validate_magic(data)?;
        let mut view = BufferView::new(&data[COPY_MAGIC.len()..]);

        let flags = view.read_i32()?;
        let ext_len = view.read_i32()?;
        if ext_len < 0 {
            return Err(Error::InvalidHeader {
                reason: "negative extension length",
            });
        }
        let extension = view.read_bytes(ext_len as usize)?.to_vec();
        let consumed = COPY_MAGIC.len() + view.consumed();
        Ok((
            PgBinaryHeader { flags, extension },
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
    if data.len() < COPY_MAGIC.len() {
        return Err(Error::UnexpectedEof {
            expected: COPY_MAGIC.len(),
            available: data.len(),
        });
    }
    if &data[..COPY_MAGIC.len()] != COPY_MAGIC.as_slice() {
        return Err(Error::InvalidMagic);
    }
    Ok(())
}
