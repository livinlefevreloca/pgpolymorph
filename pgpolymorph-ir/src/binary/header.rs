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
    pub fn parse(view: &mut BufferView<'_>) -> Result<Self> {
        validate_magic(view)?;

        let flags = view.read_i32()?;
        let ext_len = view.read_i32()?;
        if ext_len < 0 {
            return Err(Error::InvalidHeader {
                reason: "negative extension length",
            });
        }
        let extension = view.read_bytes(ext_len as usize)?.to_vec();
        Ok(PgBinaryHeader { flags, extension })
    }

    pub fn write_to(buf: &mut Vec<u8>) {
        buf.extend_from_slice(COPY_MAGIC);
        buf.extend_from_slice(&0i32.to_be_bytes());
        buf.extend_from_slice(&0i32.to_be_bytes());
    }
}

fn validate_magic(view: &mut BufferView<'_>) -> Result<()> {
    let magic = view.read_bytes(COPY_MAGIC.len())?;
    if magic != COPY_MAGIC.as_slice() {
        return Err(Error::InvalidMagic);
    }
    Ok(())
}
