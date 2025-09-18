use uvoxid::UvoxId;
use serde_json;

#[test]
fn serialize_and_deserialize_json() {
    let id = UvoxId::new(0, 6_371_000_000, 12_345_678, -98_765_432);

    let json = serde_json::to_string(&id).unwrap();
    let back: UvoxId = serde_json::from_str(&json).unwrap();

    assert_eq!(id, back);
}

#[test]
fn serialize_and_deserialize_cbor() {
    let id = UvoxId::new(42, 123, 456, -789);

    // Make a buffer to hold the CBOR bytes
    let mut buf = Vec::new();
    ciborium::ser::into_writer(&id, &mut buf).unwrap();

    // Now read it back
    let back: UvoxId = ciborium::de::from_reader(buf.as_slice()).unwrap();

    assert_eq!(id, back);
}
