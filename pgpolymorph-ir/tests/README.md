# pgpolymorph-ir tests

Integration tests for the API defined in [`SPEC.md`](../SPEC.md). Written **before**
implementation so expected behavior can be reviewed independently.

The library crate is a stub; `cargo test` will not pass until `decode`, `encode`,
IR types, and error variants are implemented.

## Layout

| File | Purpose |
|------|---------|
| [`decode_golden.rs`](decode_golden.rs) | Decode golden binaries → expected IR |
| [`encode_roundtrip.rs`](encode_roundtrip.rs) | `encode(decode(blob)) == blob` |
| [`errors.rs`](errors.rs) | Invalid magic, truncation, schema mismatches |
| [`incremental.rs`](incremental.rs) | `Decoder` / `Encoder` vs one-shot API |
| [`utils.rs`](utils.rs) | Fixture loaders, schema mapping, assertion helpers |
| [`fixtures/`](fixtures/) | Golden COPY binary files |

## Expected IR conventions

Temporal values follow **Unix-standard IR units** (see SPEC):

- `Date`: days since 1970-01-01
- `Timestamp` / `Timestamptz`: microseconds since Unix epoch (UTC)
- `Time`: microseconds since midnight

Expected IR values live inline in [`decode_golden.rs`](decode_golden.rs) at each test.
