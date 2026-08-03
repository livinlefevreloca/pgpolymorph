//! COPY binary field envelope: `int32` length prefix + optional payload.

use std::ops::{Deref, DerefMut};

use crate::binary::buffer_view::BufferView;
use crate::binary::constants;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FieldCell<'a> {
    pub is_null: bool,
    pub payload: BufferView<'a>,
}

/// Cursor over a COPY tuple stream (field count + length-prefixed fields).
pub(crate) struct FieldReader<'a> {
    view: BufferView<'a>,
}

impl<'a> FieldReader<'a> {
    pub fn from_view(view: BufferView<'a>) -> Self {
        Self { view }
    }

    pub fn read_field(&mut self) -> crate::error::Result<FieldCell<'a>> {
        let len = self.view.read_i32()? as i64;
        if len == i64::from(constants::COPY_FIELD_NULL) {
            return Ok(FieldCell {
                is_null: true,
                payload: BufferView::empty(),
            });
        }
        if len < 0 {
            return Err(crate::error::Error::FieldTooLarge { len });
        }
        let payload = self.view.take_n_and_project_view(len as usize)?;
        Ok(FieldCell {
            is_null: false,
            payload,
        })
    }
}

impl<'a> Deref for FieldReader<'a> {
    type Target = BufferView<'a>;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl<'a> DerefMut for FieldReader<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view
    }
}
