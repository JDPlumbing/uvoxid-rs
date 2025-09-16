use uvoxid::{encode_uvoxid, decode_uvoxid};
use num_bigint::BigUint;

fn main() {
    let r_um: u64 = 123456789;
    let lat_microdeg: i64 = 12345678;
    let lon_microdeg: i64 = -98765432;

    let id: BigUint = encode_uvoxid(r_um, lat_microdeg, lon_microdeg);
    println!("Encoded UVoxID: {}", id);

    // ✅ Borrow id instead of moving it
    let (r2, lat2, lon2) = decode_uvoxid(&id);
    println!("Decoded: r={}µm, lat={}µ°, lon={}µ°", r2, lat2, lon2);
}
