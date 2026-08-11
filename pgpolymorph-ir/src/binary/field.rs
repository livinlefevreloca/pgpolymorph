//! COPY binary field envelope: `int32` length prefix + optional payload.

use crate::binary::buffer_view::BufferView;
use crate::binary::constants;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FieldCell<'a> {
    pub is_null: bool,
    pub payload: BufferView<'a>,
}

/// Cursor over a COPY tuple stream (field count + length-prefixed fields).
pub(crate) struct FieldReader<'a> {
    data: BufferView<'a>,
}

impl<'a> From<BufferView<'a>> for FieldReader<'a> {
    fn from(data: BufferView<'a>) -> Self {
        Self { data }
    }
}

impl<'a> FieldReader<'a> {
    pub fn remaining(&self) -> usize {
        self.data.remaining()
    }

    pub fn read_field_count(&mut self) -> crate::error::Result<i16> {
        self.data.read_be()
    }

    pub fn peek_field_count(&self) -> crate::error::Result<i16> {
        self.data.peek_be()
    }

    pub fn read_field(&mut self) -> crate::error::Result<FieldCell<'a>> {
        let len = self.data.read_be::<i32>()? as i64;
        if len == i64::from(constants::COPY_FIELD_NULL) {
            return Ok(FieldCell {
                is_null: true,
                payload: BufferView::empty(),
            });
        }
        if len < 0 {
            return Err(crate::error::Error::FieldTooLarge { len });
        }
        let payload = self.data.read_n_and_project_view(len as usize)?;
        Ok(FieldCell {
            is_null: false,
            payload,
        })
    }
}
