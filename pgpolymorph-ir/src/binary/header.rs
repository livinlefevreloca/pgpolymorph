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
        const MAGIC_LEN: usize = COPY_MAGIC.len();
        let magic = data.read_fixed::<MAGIC_LEN>()?;
        if magic.as_slice() != COPY_MAGIC.as_slice() {
            return Err(Error::InvalidMagic);
        }

        let flags = data.read_be::<i32>()?;
        let ext_len = data.read_be::<i32>()?;
        if ext_len < 0 {
            return Err(Error::InvalidHeader {
                reason: "negative extension length",
            });
        }

        let extension = if ext_len == 0 {
            Vec::new()
        } else {
            let mut ext = data.read_n_and_project_view(ext_len as usize)?;
            ext.read_remaining()?.to_vec()
        };

        Ok(PgBinaryHeader { flags, extension })
    }

    pub fn write_to(buf: &mut Vec<u8>) {
        buf.extend_from_slice(COPY_MAGIC);
        buf.extend_from_slice(&0i32.to_be_bytes());
        buf.extend_from_slice(&0i32.to_be_bytes());
    }
}
