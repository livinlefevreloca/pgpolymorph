//! Incremental [`Decoder`] and [`Encoder`] API (row-at-a-time).

mod utils;

use pgpolymorph_ir::value::types;
use pgpolymorph_ir::value::{CopyBatch, Row, Value};
use pgpolymorph_ir::{decode, encode, Decoder, Encoder};

#[test]
fn decoder_yields_same_rows_as_decode() {
    let fixture = "int32";
    let schema = utils::schema_for_fixture(fixture);
    let blob = utils::load_fixture(fixture);
    let expected = CopyBatch {
        rows: vec![
            Row {
                values: vec![Value::Int4(types::PgInt4::new(-1))],
            },
            Row {
                values: vec![Value::Int4(types::PgInt4::new(0))],
            },
            Row {
                values: vec![Value::Int4(types::PgInt4::new(1))],
            },
        ],
    };

    let batch = decode(&schema, &blob).unwrap();

    let mut decoder = Decoder::new(&schema, &blob).unwrap();
    let mut rows = Vec::new();
    while let Some(row) = decoder.next_row().unwrap() {
        rows.push(row);
    }
    assert!(decoder.next_row().unwrap().is_none());

    assert_eq!(batch.rows.len(), expected.rows.len());
    utils::assert_batches_eq(&expected, &CopyBatch { rows: rows.clone() });
    assert_eq!(batch.rows, rows);
}

#[test]
fn encoder_builds_same_bytes_as_encode() {
    let fixture = "bool";
    let schema = utils::schema_for_fixture(fixture);
    let blob = utils::load_fixture(fixture);
    let batch = decode(&schema, &blob).unwrap();

    let mut encoder = Encoder::new(&schema);
    for row in &batch.rows {
        encoder.write_row(row).unwrap();
    }
    let incremental = encoder.finish().unwrap();
    let one_shot = encode(&schema, &batch).unwrap();

    assert_eq!(one_shot, incremental);
}
