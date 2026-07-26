//! Shared helpers for integration tests (fixtures, schemas).

use std::fs;
use std::path::PathBuf;

use pgpolymorph_ir::schema::{Column, PgType, Schema};
use pgpolymorph_ir::value::CopyBatch;

pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

pub fn load_fixture(name: &str) -> Vec<u8> {
    let path = fixtures_dir().join(format!("{name}.bin"));
    fs::read(&path).unwrap_or_else(|e| panic!("failed to read fixture {}: {e}", path.display()))
}

pub fn schema_for_fixture(name: &str) -> Schema {
    let nullable = name.ends_with("_nullable");
    let base = name.strip_suffix("_nullable").unwrap_or(name);

    let (col_name, ty) = column_for_fixture(base);
    Schema {
        columns: vec![Column {
            name: col_name.to_string(),
            ty,
            nullable,
        }],
    }
}

fn column_for_fixture(base: &str) -> (&'static str, PgType) {
    match base {
        "bool" => ("bool", PgType::Bool),
        "uint8" => ("uint8", PgType::Int2),
        "uint16" => ("uint16", PgType::Int4),
        "uint32" => ("uint32", PgType::Int8),
        "int8" => ("int8", PgType::Int2),
        "int16" => ("int16", PgType::Int2),
        "int32" => ("int32", PgType::Int4),
        "int64" => ("int64", PgType::Int8),
        "float32" => ("float32", PgType::Float4),
        "float64" => ("float64", PgType::Float8),
        "string" | "large_string" => ("string", PgType::Text),
        "binary" | "large_binary" => ("binary", PgType::Bytea),
        "date32" => ("date32", PgType::Date),
        "time_s" | "time_ms" | "time_us" => ("time", PgType::Time),
        "timestamp_s_notz" | "timestamp_ms_notz" | "timestamp_us_notz" => {
            ("timestamp", PgType::Timestamp)
        }
        "timestamp_s_tz" | "timestamp_ms_tz" | "timestamp_us_tz" => {
            ("timestamp", PgType::Timestamptz)
        }
        "duration_s" | "duration_ms" | "duration_us" => ("duration", PgType::Interval),
        "uuid" => ("id", PgType::Uuid),
        "money" => ("price", PgType::Money),
        "oid" => ("relid", PgType::Oid),
        "numeric" => ("n", PgType::Numeric),
        "int32_matrix" => ("matrix", PgType::Array(Box::new(PgType::Int4))),
        name if name.starts_with("list_") => {
            let inner = name.strip_prefix("list_").unwrap();
            let inner = inner.strip_prefix("nullable_").unwrap_or(inner);
            let (_, inner_ty) = column_for_fixture(inner);
            ("list", PgType::Array(Box::new(inner_ty)))
        }
        name if name.starts_with("list_nullable_") => {
            let inner = name.strip_prefix("list_nullable_").unwrap();
            let (_, inner_ty) = column_for_fixture(inner);
            ("list", PgType::Array(Box::new(inner_ty)))
        }
        other => panic!("no schema mapping for fixture: {other}"),
    }
}

pub fn assert_batches_eq(expected: &CopyBatch, actual: &CopyBatch) {
    assert_eq!(
        expected.rows.len(),
        actual.rows.len(),
        "row count mismatch"
    );
    for (i, (exp_row, act_row)) in expected.rows.iter().zip(actual.rows.iter()).enumerate() {
        assert_eq!(
            exp_row.values.len(),
            act_row.values.len(),
            "column count mismatch in row {i}"
        );
        for (j, (exp_val, act_val)) in exp_row.values.iter().zip(act_row.values.iter()).enumerate()
        {
            assert_eq!(exp_val, act_val, "value mismatch at row {i} column {j}");
        }
    }
}
