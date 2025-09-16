use num_bigint::BigUint;
use num_traits::ToPrimitive;

/// Encode spherical coordinates into a 192-bit UVoxID integer.
///
/// Fields (fixed units):
/// - `r_um`: radius in micrometers (µm)
/// - `lat_microdeg`: latitude in millionths of a degree (-90e6 to +90e6)
/// - `lon_microdeg`: longitude in millionths of a degree (-180e6 to +180e6)
///
/// Returns: 192-bit integer `BigUint`.
pub fn encode_uvoxid(r_um: u64, lat_microdeg: i64, lon_microdeg: i64) -> BigUint {
    let lat_enc = (lat_microdeg + 90_000_000) as u64;
    let lon_enc = (lon_microdeg + 180_000_000) as u64;

    // Pack fields: [ r (64b) | lat (64b) | lon (64b) ]
    (BigUint::from(r_um) << 128u32)
        | (BigUint::from(lat_enc) << 64u32)
        | BigUint::from(lon_enc)
}

/// Decode a 192-bit UVoxID back into spherical coordinates.
///
/// Returns tuple: (r_um, lat_microdeg, lon_microdeg).
pub fn decode_uvoxid(uvoxid: &BigUint) -> (u64, i64, i64) {
    let mask64 = BigUint::from(u64::MAX);

    let lon_enc = (uvoxid & &mask64).to_u64().unwrap();
    let lat_enc = ((uvoxid >> 64u32) & &mask64).to_u64().unwrap();
    let r_um    = ((uvoxid >> 128u32) & &mask64).to_u64().unwrap();

    let lat_microdeg = lat_enc as i64 - 90_000_000;
    let lon_microdeg = lon_enc as i64 - 180_000_000;

    (r_um, lat_microdeg, lon_microdeg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_encode_decode() {
        let r = 123456789u64;
        let lat = 1_234_567i64;
        let lon = -12_345_678i64;

        let id = encode_uvoxid(r, lat, lon);
        let (r2, lat2, lon2) = decode_uvoxid(&id);

        assert_eq!(r, r2);
        assert_eq!(lat, lat2);
        assert_eq!(lon, lon2);
    }

    #[test]
    fn works_at_edges() {
        let r = u64::MAX;
        let lat = 90_000_000;
        let lon = -180_000_000;

        let id = encode_uvoxid(r, lat, lon);
        let (r2, lat2, lon2) = decode_uvoxid(&id);

        assert_eq!(r, r2);
        assert_eq!(lat, lat2);
        assert_eq!(lon, lon2);
    }
}
