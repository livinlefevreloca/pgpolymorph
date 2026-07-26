//! PostgreSQL type OID helpers.

use crate::schema::PgType;

pub fn oid_to_pg_type(oid: u32) -> Option<PgType> {
    PgType::from_oid(oid)
}
