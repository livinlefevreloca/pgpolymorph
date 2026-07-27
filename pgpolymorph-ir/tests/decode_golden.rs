//! Decode golden COPY binary fixtures and assert expected IR.
//!
//! Expected values are spelled out at each test site. Temporal fields use
//! Unix-standard IR units per `SPEC.md`.

mod utils;

use pgpolymorph_ir::decode;
use pgpolymorph_ir::schema::PgType;
use pgpolymorph_ir::value::pgtypes;
use pgpolymorph_ir::value::{CopyBatch, Row, Value};
use pgpolymorph_ir::COPY_MAGIC;

const TIMESTAMP_US: i64 = 1_676_142_874 * 1_000_000;
const TIME_US: i64 = (24 * 60 * 60 - 1) * 1_000_000;
const DATE32: i32 = (1 << 16) - 1;
const DURATION_US: i64 = 60 * 1_000_000;
const STRING: &str = "some data! ";
const BINARY: &[u8] = b"some data! ";
const UUID: [u8; 16] = [
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6,
    0xb7,
];

fn row(value: Value) -> Row {
    Row {
        values: vec![value],
    }
}

fn batch(rows: Vec<Row>) -> CopyBatch {
    CopyBatch { rows }
}

fn interval_micros(micros: i64) -> Value {
    Value::Interval(pgtypes::PgInterval::new(micros, 0, 0))
}

fn int32_array(vals: &[i32]) -> Value {
    Value::Array(pgtypes::PgArray::new(
        PgType::Int4,
        vec![pgtypes::ArrayDimension {
            length: vals.len() as i32,
            lower_bound: 1,
        }],
        vals.iter()
            .map(|&n| Value::Int4(pgtypes::PgInt4::new(n)))
            .collect(),
    ))
}

fn bool_array(vals: &[bool]) -> Value {
    Value::Array(pgtypes::PgArray::new(
        PgType::Bool,
        vec![pgtypes::ArrayDimension {
            length: vals.len() as i32,
            lower_bound: 1,
        }],
        vals.iter()
            .map(|&b| Value::Bool(pgtypes::PgBool::new(b)))
            .collect(),
    ))
}

fn int32_matrix_array(vals: &[i32]) -> Value {
    Value::Array(pgtypes::PgArray::new(
        PgType::Int4,
        vec![
            pgtypes::ArrayDimension {
                length: 2,
                lower_bound: 1,
            },
            pgtypes::ArrayDimension {
                length: 3,
                lower_bound: 1,
            },
        ],
        vals.iter()
            .map(|&n| Value::Int4(pgtypes::PgInt4::new(n)))
            .collect(),
    ))
}

fn assert_decodes_to(fixture: &str, expected: CopyBatch) {
    let schema = utils::schema_for_fixture(fixture);
    let blob = utils::load_fixture(fixture);
    let actual = decode(&schema, &blob).expect("decode should succeed");
    utils::assert_batches_eq(&expected, &actual);
}

#[test]
fn decode_golden_bool() {
    assert_decodes_to(
        "bool",
        batch(vec![
            row(Value::Bool(pgtypes::PgBool::new(true))),
            row(Value::Bool(pgtypes::PgBool::new(false))),
        ]),
    );
}

#[test]
fn decode_golden_bool_nullable() {
    assert_decodes_to(
        "bool_nullable",
        batch(vec![
            row(Value::Bool(pgtypes::PgBool::new(true))),
            row(Value::Bool(pgtypes::PgBool::new(false))),
            row(Value::Null),
        ]),
    );
}

#[test]
fn decode_golden_int8() {
    assert_decodes_to(
        "int8",
        batch(vec![
            row(Value::Int2(pgtypes::PgInt2::new(-1))),
            row(Value::Int2(pgtypes::PgInt2::new(0))),
            row(Value::Int2(pgtypes::PgInt2::new(1))),
        ]),
    );
}

#[test]
fn decode_golden_int16() {
    assert_decodes_to(
        "int16",
        batch(vec![
            row(Value::Int2(pgtypes::PgInt2::new(-1))),
            row(Value::Int2(pgtypes::PgInt2::new(0))),
            row(Value::Int2(pgtypes::PgInt2::new(1))),
        ]),
    );
}

#[test]
fn decode_golden_int32() {
    assert_decodes_to(
        "int32",
        batch(vec![
            row(Value::Int4(pgtypes::PgInt4::new(-1))),
            row(Value::Int4(pgtypes::PgInt4::new(0))),
            row(Value::Int4(pgtypes::PgInt4::new(1))),
        ]),
    );
}

#[test]
fn decode_golden_int32_nullable() {
    assert_decodes_to(
        "int32_nullable",
        batch(vec![
            row(Value::Int4(pgtypes::PgInt4::new(-1))),
            row(Value::Int4(pgtypes::PgInt4::new(0))),
            row(Value::Int4(pgtypes::PgInt4::new(1))),
            row(Value::Null),
        ]),
    );
}

#[test]
fn decode_golden_int64() {
    assert_decodes_to(
        "int64",
        batch(vec![
            row(Value::Int8(pgtypes::PgInt8::new(-1))),
            row(Value::Int8(pgtypes::PgInt8::new(0))),
            row(Value::Int8(pgtypes::PgInt8::new(1))),
        ]),
    );
}

#[test]
fn decode_golden_float32() {
    assert_decodes_to(
        "float32",
        batch(vec![
            row(Value::Float4(pgtypes::PgFloat4::new(-1.0))),
            row(Value::Float4(pgtypes::PgFloat4::new(0.0))),
            row(Value::Float4(pgtypes::PgFloat4::new(1.0))),
            row(Value::Float4(pgtypes::PgFloat4::new(f32::INFINITY))),
        ]),
    );
}

#[test]
fn decode_golden_float64() {
    assert_decodes_to(
        "float64",
        batch(vec![
            row(Value::Float8(pgtypes::PgFloat8::new(-1.0))),
            row(Value::Float8(pgtypes::PgFloat8::new(0.0))),
            row(Value::Float8(pgtypes::PgFloat8::new(1.0))),
            row(Value::Float8(pgtypes::PgFloat8::new(f64::INFINITY))),
        ]),
    );
}

#[test]
fn decode_golden_string() {
    assert_decodes_to(
        "string",
        batch(vec![
            row(Value::Text(pgtypes::PgText::new(""))),
            row(Value::Text(pgtypes::PgText::new(STRING))),
        ]),
    );
}

#[test]
fn decode_golden_string_nullable() {
    assert_decodes_to(
        "string_nullable",
        batch(vec![
            row(Value::Text(pgtypes::PgText::new(""))),
            row(Value::Text(pgtypes::PgText::new(STRING))),
            row(Value::Null),
        ]),
    );
}

#[test]
fn decode_golden_binary() {
    assert_decodes_to(
        "binary",
        batch(vec![
            row(Value::Bytea(pgtypes::PgBytea::new(vec![]))),
            row(Value::Bytea(pgtypes::PgBytea::new(BINARY))),
        ]),
    );
}

#[test]
fn decode_golden_binary_nullable() {
    assert_decodes_to(
        "binary_nullable",
        batch(vec![
            row(Value::Bytea(pgtypes::PgBytea::new(vec![]))),
            row(Value::Bytea(pgtypes::PgBytea::new(BINARY))),
            row(Value::Null),
        ]),
    );
}

#[test]
fn decode_golden_date32() {
    assert_decodes_to(
        "date32",
        batch(vec![
            row(Value::Date(pgtypes::PgDate::new(0))),
            row(Value::Date(pgtypes::PgDate::new(-DATE32))),
            row(Value::Date(pgtypes::PgDate::new(DATE32))),
        ]),
    );
}

#[test]
fn decode_golden_date32_nullable() {
    assert_decodes_to(
        "date32_nullable",
        batch(vec![
            row(Value::Date(pgtypes::PgDate::new(0))),
            row(Value::Date(pgtypes::PgDate::new(-DATE32))),
            row(Value::Date(pgtypes::PgDate::new(DATE32))),
            row(Value::Null),
        ]),
    );
}

#[test]
fn decode_golden_time_us() {
    assert_decodes_to(
        "time_us",
        batch(vec![
            row(Value::Time(pgtypes::PgTime::new(0))),
            row(Value::Time(pgtypes::PgTime::new(1))),
            row(Value::Time(pgtypes::PgTime::new(TIME_US))),
        ]),
    );
}

#[test]
fn decode_golden_timestamp_us_notz() {
    assert_decodes_to(
        "timestamp_us_notz",
        batch(vec![
            row(Value::Timestamp(pgtypes::PgTimestamp::new(0))),
            row(Value::Timestamp(pgtypes::PgTimestamp::new(1))),
            row(Value::Timestamp(pgtypes::PgTimestamp::new(TIMESTAMP_US))),
        ]),
    );
}

#[test]
fn decode_golden_timestamp_us_tz() {
    assert_decodes_to(
        "timestamp_us_tz",
        batch(vec![
            row(Value::Timestamptz(pgtypes::PgTimestamptz::new(0))),
            row(Value::Timestamptz(pgtypes::PgTimestamptz::new(1))),
            row(Value::Timestamptz(pgtypes::PgTimestamptz::new(TIMESTAMP_US))),
        ]),
    );
}

#[test]
fn decode_golden_duration_us() {
    assert_decodes_to(
        "duration_us",
        batch(vec![
            row(interval_micros(0)),
            row(interval_micros(1)),
            row(interval_micros(DURATION_US)),
        ]),
    );
}

#[test]
fn decode_golden_list_int32() {
    assert_decodes_to("list_int32", batch(vec![row(int32_array(&[-1, 0, 1]))]));
}

#[test]
fn decode_golden_list_int32_nullable() {
    assert_decodes_to(
        "list_int32_nullable",
        batch(vec![row(int32_array(&[-1, 0, 1]))]),
    );
}

#[test]
fn decode_golden_list_bool() {
    assert_decodes_to("list_bool", batch(vec![row(bool_array(&[true, false]))]));
}

#[test]
fn decode_golden_uuid() {
    assert_decodes_to("uuid", batch(vec![row(Value::Uuid(pgtypes::PgUuid::new(UUID)))]));
}

#[test]
fn decode_golden_uuid_nullable() {
    assert_decodes_to(
        "uuid_nullable",
        batch(vec![row(Value::Uuid(pgtypes::PgUuid::new(UUID))), row(Value::Null)]),
    );
}

#[test]
fn decode_golden_money() {
    assert_decodes_to(
        "money",
        batch(vec![row(Value::Money(pgtypes::PgMoney::new(12_499)))]),
    );
}

#[test]
fn decode_golden_oid() {
    assert_decodes_to("oid", batch(vec![row(Value::Oid(pgtypes::PgOid::new(3802)))]));
}

#[test]
fn decode_golden_numeric() {
    assert_decodes_to(
        "numeric",
        batch(vec![row(Value::Numeric(pgtypes::PgNumeric::zero()))]),
    );
}

#[test]
fn decode_golden_int32_matrix() {
    assert_decodes_to(
        "int32_matrix",
        batch(vec![row(int32_matrix_array(&[1, 2, 3, 4, 5, 6]))]),
    );
}

#[test]
fn fixture_begins_with_copy_magic() {
    let blob = utils::load_fixture("bool");
    assert_eq!(&blob[..11], COPY_MAGIC);
}

#[test]
fn fixture_ends_with_footer_sentinel() {
    let blob = utils::load_fixture("bool");
    assert_eq!(blob[blob.len() - 2..], [0xFF, 0xFF]);
}

#[test]
fn timestamptz_decodes_to_timestamptz_ir_variant() {
    let schema = utils::schema_for_fixture("timestamp_us_tz");
    let blob = utils::load_fixture("timestamp_us_tz");
    let batch = decode(&schema, &blob).expect("decode");
    assert!(matches!(
        batch.rows[0].values[0],
        Value::Timestamptz(_)
    ));
}
