//! Field envelope: `int32` length prefix + optional payload.

use std::ops::{Deref, DerefMut};

use crate::binary::buffer_view::BufferView;
use crate::binary::constants;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FieldCell<'a> {
    pub is_null: bool,
    pub payload: &'a [u8],
}

pub(crate) struct FieldReader<'a> {
    view: BufferView<'a>,
}

impl<'a> FieldReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            view: BufferView::new(data),
        }
    }

    pub fn remaining(&self) -> usize {
        self.view.remaining()
    }

    pub fn read_field(&mut self) -> crate::error::Result<FieldCell<'a>> {
        let len = self.view.read_i32()? as i64;
        if len == i64::from(constants::COPY_FIELD_NULL) {
            return Ok(FieldCell {
                is_null: true,
                payload: &[],
            });
        }
        if len < 0 {
            return Err(crate::error::Error::FieldTooLarge { len });
        }
        let payload = self.view.read_bytes(len as usize)?;
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
