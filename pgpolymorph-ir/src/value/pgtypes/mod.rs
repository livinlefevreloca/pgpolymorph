//! Typed intermediate representation for each PostgreSQL scalar and container type.

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
mod varchar;

pub use array::{ArrayDimension, PgArray};
pub use bool::PgBool;
pub use bytea::PgBytea;
pub use char::PgChar;
pub use float::{PgFloat4, PgFloat8};
pub use int::{PgInt2, PgInt4, PgInt8};
pub use interval::PgInterval;
pub use json::{PgJson, PgJsonb};
/// Fixed-scale currency amount in micro-dollars (PostgreSQL `money` encoding).
pub use money::PgMoney;
pub use name::PgName;
pub use numeric::{NumericSign, PgNumeric};
pub use oid::PgOid;
pub use temporal::{PgDate, PgTime, PgTimestamp, PgTimestamptz, PgTimetz};
pub use text::PgText;
pub use uuid::PgUuid;
pub use varchar::PgVarchar;
