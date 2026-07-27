//! Encode(decode(blob)) roundtrip against golden fixtures.

mod utils;

use pgpolymorph_ir::value::pgtypes;
use pgpolymorph_ir::value::{PgBatch, PgRow, PgValue};
use pgpolymorph_ir::{decode, encode};

macro_rules! roundtrip_test {
    ($name:ident, $fixture:literal) => {
        #[test]
        fn $name() {
            let schema = utils::schema_for_fixture($fixture);
            let original = utils::load_fixture($fixture);

            let batch = decode(&schema, &original).expect("decode");
            let reencoded = encode(&schema, &batch).expect("encode");

            assert_eq!(
                original, reencoded,
                "roundtrip must reproduce identical COPY binary bytes"
            );
        }
    };
}

roundtrip_test!(roundtrip_bool, "bool");
roundtrip_test!(roundtrip_int32, "int32");
roundtrip_test!(roundtrip_float64, "float64");
roundtrip_test!(roundtrip_string, "string");
roundtrip_test!(roundtrip_date32, "date32");
roundtrip_test!(roundtrip_timestamp_us_notz, "timestamp_us_notz");
roundtrip_test!(roundtrip_timestamp_us_tz, "timestamp_us_tz");
roundtrip_test!(roundtrip_list_int32, "list_int32");
roundtrip_test!(roundtrip_uuid, "uuid");
roundtrip_test!(roundtrip_money, "money");
roundtrip_test!(roundtrip_int32_matrix, "int32_matrix");

#[test]
fn roundtrip_preserves_nullable_rows() {
    let fixture = "int32_nullable";
    let schema = utils::schema_for_fixture(fixture);
    let original = utils::load_fixture(fixture);

    let batch = decode(&schema, &original).unwrap();
    let reencoded = encode(&schema, &batch).unwrap();
    assert_eq!(original, reencoded);
}

#[test]
fn encode_then_decode_matches_original_ir() {
    let fixture = "bool";
    let schema = utils::schema_for_fixture(fixture);
    let original = utils::load_fixture(fixture);
    let expected_ir = PgBatch {
        rows: vec![
            PgRow {
                values: vec![PgValue::Bool(pgtypes::PgBool::new(true))],
            },
            PgRow {
                values: vec![PgValue::Bool(pgtypes::PgBool::new(false))],
            },
        ],
    };

    let batch = decode(&schema, &original).unwrap();
    let bytes = encode(&schema, &batch).unwrap();
    let again = decode(&schema, &bytes).unwrap();

    utils::assert_batches_eq(&expected_ir, &again);
}
