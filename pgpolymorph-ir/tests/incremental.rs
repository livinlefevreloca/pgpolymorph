//! Incremental [`Decoder`] and [`Encoder`] API (row-at-a-time).

mod utils;

use pgpolymorph_ir::value::pgtypes;
use pgpolymorph_ir::value::{PgBatch, PgRow, PgValue};
use pgpolymorph_ir::{decode, encode, Decoder, Encoder};

#[test]
fn decoder_yields_same_rows_as_decode() {
    let fixture = "int32";
    let schema = utils::schema_for_fixture(fixture);
    let blob = utils::load_fixture(fixture);
    let expected = PgBatch {
        rows: vec![
            PgRow {
                values: vec![PgValue::Int4(pgtypes::PgInt4::new(-1))],
            },
            PgRow {
                values: vec![PgValue::Int4(pgtypes::PgInt4::new(0))],
            },
            PgRow {
                values: vec![PgValue::Int4(pgtypes::PgInt4::new(1))],
            },
        ],
    };

    let batch = decode(&schema, &blob).unwrap();

    let decoder = Decoder::new(&schema, &blob).unwrap();
    let rows: Vec<_> = decoder.map(|row| row.unwrap()).collect();

    assert_eq!(batch.rows.len(), expected.rows.len());
    utils::assert_batches_eq(&expected, &PgBatch { rows: rows.clone() });
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
