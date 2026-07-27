//! Typed intermediate representation for each PostgreSQL scalar and container type.

macro_rules! define_pg_scalar {
    ($(#[$meta:meta])*, $name:ident, $ty:ty) => {
        define_pg_scalar!(@inner $(#[$meta])*, $name, $ty, value);
    };
    ($name:ident, $ty:ty) => {
        define_pg_scalar!(@inner, $name, $ty, value);
    };
    ($name:ident, $ty:ty, $field:ident) => {
        define_pg_scalar!(@inner, $name, $ty, $field);
    };
    ($(#[$meta:meta])*, $name:ident, $ty:ty, $field:ident) => {
        define_pg_scalar!(@inner $(#[$meta])*, $name, $ty, $field);
    };
    (@inner $(#[$meta:meta])*, $name:ident, $ty:ty, $field:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name {
            pub $field: $ty,
        }

        impl $name {
            pub const fn new($field: $ty) -> Self {
                Self { $field }
            }
        }

        impl From<$ty> for $name {
            fn from($field: $ty) -> Self {
                Self::new($field)
            }
        }
    };
}

macro_rules! define_pg_float {
    ($name:ident, $ty:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name {
            pub value: $ty,
        }

        impl $name {
            pub const fn new(value: $ty) -> Self {
                Self { value }
            }
        }

        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                Self::new(value)
            }
        }
    };
}

mod array;
mod bool;
mod bytea;
mod char;
mod float;
mod int;
mod interval;
mod json;
mod money;
mod name;
mod numeric;

pub(crate) use numeric::{decode_numeric, encode_numeric};
mod oid;
mod temporal;
mod text;
mod uuid;

pub use array::{ArrayDimension, PgArray};
pub use bool::PgBool;
pub use bytea::PgBytea;
pub use char::PgChar;
pub use float::{PgFloat4, PgFloat8};
pub use int::{PgInt2, PgInt4, PgInt8};
pub use interval::PgInterval;
pub use json::{PgJson, PgJsonb};
/// Fixed-scale currency amount in micro-dollars (PostgreSQL `money` wire encoding).
pub use money::PgMoney;
pub use name::PgName;
pub use numeric::{NumericSign, PgNumeric};
pub use oid::PgOid;
pub use temporal::{PgDate, PgTime, PgTimestamp, PgTimestamptz, PgTimetz};
pub use text::PgText;
pub use uuid::PgUuid;
