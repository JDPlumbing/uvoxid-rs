use uvoxid::{encode_uvoxid, decode_uvoxid};
use num_bigint::BigUint;

#[test]
fn roundtrip_encode_decode() {
    let r: u64 = 42;
    let lat: i64 = 1_000_000;   // ~1 degree
    let lon: i64 = -2_000_000;  // ~-2 degrees

    let id: BigUint = encode_uvoxid(r, lat, lon);

    // ✅ Borrow id instead of moving it
    let (r2, lat2, lon2) = decode_uvoxid(&id);

    assert_eq!(r, r2);
    assert_eq!(lat, lat2);
    assert_eq!(lon, lon2);
}

#[test]
fn different_values() {
    let r: u64 = 123456789;
    let lat: i64 = 12_345_678;
    let lon: i64 = -98_765_432;

    let id: BigUint = encode_uvoxid(r, lat, lon);

    // ✅ Borrow id here too
    let (decoded_r, decoded_lat, decoded_lon) = decode_uvoxid(&id);

    assert_eq!(r, decoded_r);
    assert_eq!(lat, decoded_lat);
    assert_eq!(lon, decoded_lon);
}
