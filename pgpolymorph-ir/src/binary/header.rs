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
    pub fn parse(data: &mut BufferView<'_>) -> Result<Self> {
        validate_magic(data)?;

        let flags = data.read_i32()?;
        let ext_len = data.read_i32()?;
        if ext_len < 0 {
            return Err(Error::InvalidHeader {
                reason: "negative extension length",
            });
        }
        let extension = read_extension(data, ext_len as usize)?;
        Ok(PgBinaryHeader { flags, extension })
    }

    pub fn write_to(buf: &mut Vec<u8>) {
        buf.extend_from_slice(COPY_MAGIC);
        buf.extend_from_slice(&0i32.to_be_bytes());
        buf.extend_from_slice(&0i32.to_be_bytes());
    }
}

fn validate_magic(data: &mut BufferView<'_>) -> Result<()> {
    const MAGIC_LEN: usize = COPY_MAGIC.len();
    let magic = data.read_fixed::<MAGIC_LEN>()?;
    if magic.as_slice() == COPY_MAGIC.as_slice() {
        Ok(())
    } else {
        Err(Error::InvalidMagic)
    }
}

fn read_extension(data: &mut BufferView<'_>, len: usize) -> Result<Vec<u8>> {
    if len == 0 {
        return Ok(Vec::new());
    }
    let mut ext = data.read_n_and_project_view(len)?;
    Ok(ext.read_remaining()?.to_vec())
}
