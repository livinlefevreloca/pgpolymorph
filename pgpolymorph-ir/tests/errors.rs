//! Error handling for malformed or inconsistent COPY binary input.

use pgpolymorph_ir::value::pgtypes;
use pgpolymorph_ir::{decode, encode, Error};
use pgpolymorph_ir::schema::{Column, PgType, Schema};
use pgpolymorph_ir::value::{PgBatch, PgRow, PgValue};

/// Valid COPY header only (flags + extension length zero); 19 bytes total.
const TRUNCATED_HEADER: &[u8] = b"PGCOPY\n\xFF\r\n\x00\x00\x00\x00\x00\x00\x00\x00";

/// Valid header + one int32 column row, no footer sentinel.
const MISSING_FOOTER: &[u8] = &[
    0x50, 0x47, 0x43, 0x4f, 0x50, 0x59, 0x0a, 0xff, 0x0d, 0x0a, 0x00, // magic
    0x00, 0x00, 0x00, 0x00, // flags
    0x00, 0x00, 0x00, 0x00, // extension length
    0x00, 0x01, // field_count = 1
    0x00, 0x00, 0x00, 0x04, // payload length = 4
    0x00, 0x00, 0x00, 0x00, // int32 value 0
];

/// Valid single-column int32 row with footer.
const SINGLE_INT32_ROW: &[u8] = &[
    0x50, 0x47, 0x43, 0x4f, 0x50, 0x59, 0x0a, 0xff, 0x0d, 0x0a, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x01,
    0x00, 0x00, 0x00, 0x04,
    0x00, 0x00, 0x00, 0x2a, // int32 value 42
    0xff, 0xff, // footer
];

fn int32_schema(nullable: bool) -> Schema {
    Schema {
        columns: vec![Column {
            name: "int32".into(),
            ty: PgType::Int4,
            nullable,
        }],
    }
}

#[test]
fn reject_invalid_magic() {
    let schema = int32_schema(false);
    let blob = b"BAD!\n\xff\r\n\x00";
    let err = decode(&schema, blob).unwrap_err();
    assert!(matches!(err, Error::InvalidMagic));
}

#[test]
fn reject_truncated_header() {
    let schema = int32_schema(false);
    let err = decode(&schema, TRUNCATED_HEADER).unwrap_err();
    assert!(matches!(err, Error::UnexpectedEof { .. }));
}

#[test]
fn reject_missing_footer() {
    let schema = int32_schema(false);
    let err = decode(&schema, MISSING_FOOTER).unwrap_err();
    assert!(
        matches!(err, Error::InvalidFooter | Error::UnexpectedEof { .. }),
        "unexpected error: {err:?}"
    );
}

#[test]
fn reject_field_count_mismatch() {
    let schema = Schema {
        columns: vec![
            Column {
                name: "a".into(),
                ty: PgType::Int4,
                nullable: false,
            },
            Column {
                name: "b".into(),
                ty: PgType::Int4,
                nullable: false,
            },
        ],
    };
    let err = decode(&schema, SINGLE_INT32_ROW).unwrap_err();
    assert!(matches!(err, Error::FieldCountMismatch { .. }));
}

#[test]
fn reject_null_in_non_nullable_column_on_encode() {
    let schema = int32_schema(false);
    let batch = PgBatch {
        rows: vec![PgRow {
            values: vec![PgValue::Null],
        }],
    };
    let err = encode(&schema, &batch).unwrap_err();
    assert!(matches!(err, Error::UnexpectedNull { .. }));
}

#[test]
fn reject_row_with_wrong_column_count_on_encode() {
    let schema = int32_schema(false);
    let batch = PgBatch {
        rows: vec![PgRow {
            values: vec![PgValue::Int4(pgtypes::PgInt4::new(1)), PgValue::Int4(pgtypes::PgInt4::new(2))],
        }],
    };
    let err = encode(&schema, &batch).unwrap_err();
    assert!(matches!(err, Error::SchemaRowLengthMismatch { .. }));
}

#[test]
fn reject_type_mismatch_on_encode() {
    let schema = int32_schema(false);
    let batch = PgBatch {
        rows: vec![PgRow {
            values: vec![PgValue::Text(pgtypes::PgText::new("not an int"))],
        }],
    };
    let err = encode(&schema, &batch).unwrap_err();
    assert!(matches!(err, Error::TypeMismatch { .. }));
}

#[test]
fn reject_empty_input() {
    let schema = int32_schema(false);
    let err = decode(&schema, &[]).unwrap_err();
    assert!(matches!(err, Error::UnexpectedEof { .. } | Error::InvalidMagic));
}
