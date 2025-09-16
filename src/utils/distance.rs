// src/utils/distance.rs
//! Distance utilities for UVoxID.


use crate::decode_uvoxid;
use num_bigint::BigUint;

/// Linear (chord) distance between two voxels in meters.
///
/// Uses spherical law of cosines directly on (r, lat, lon).
/// Matches small-scale "straightness" intuition, and is exact for large scales.
pub fn linear_distance(uv1: &BigUint, uv2: &BigUint) -> f64 {
    let (r1_um, lat1_micro, lon1_micro) = decode_uvoxid(uv1);
    let (r2_um, lat2_micro, lon2_micro) = decode_uvoxid(uv2);

    // Convert to meters and radians
    let r1_m = r1_um as f64 * 1e-6;
    let r2_m = r2_um as f64 * 1e-6;
    let lat1 = (lat1_micro as f64 / 1e6).to_radians();
    let lon1 = (lon1_micro as f64 / 1e6).to_radians();
    let lat2 = (lat2_micro as f64 / 1e6).to_radians();
    let lon2 = (lon2_micro as f64 / 1e6).to_radians();

    // Central angle (γ) via spherical law of cosines
    let mut cos_gamma = lat1.sin() * lat2.sin() + lat1.cos() * lat2.cos() * (lon2 - lon1).cos();
    cos_gamma = cos_gamma.clamp(-1.0, 1.0);

    let _gamma = cos_gamma.acos();

    // Law of cosines for chord length
    (r1_m.powi(2) + r2_m.powi(2) - 2.0 * r1_m * r2_m * cos_gamma).sqrt()
}

/// Great-circle (surface) distance in meters.
///
/// Assumes both voxels lie on the surface of the same sphere,
/// uses the average radius of r1 and r2.
pub fn haversine_distance(uv1: &BigUint, uv2: &BigUint) -> f64 {
    let (r1_um, lat1_micro, lon1_micro) = decode_uvoxid(uv1);
    let (r2_um, lat2_micro, lon2_micro) = decode_uvoxid(uv2);

    let r_m = ((r1_um + r2_um) as f64 / 2.0) * 1e-6;

    let lat1 = (lat1_micro as f64 / 1e6).to_radians();
    let lon1 = (lon1_micro as f64 / 1e6).to_radians();
    let lat2 = (lat2_micro as f64 / 1e6).to_radians();
    let lon2 = (lon2_micro as f64 / 1e6).to_radians();

    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;

    let a = (dlat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    r_m * c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode_uvoxid;

    #[test]
    fn test_linear_distance() {
        let r_um = 6_371_000_000_000; // Earth radius in µm
        let uv1 = encode_uvoxid(r_um, 0, 0);
        let uv2 = encode_uvoxid(r_um, 0, 1_000_000); // 1° east

        let d = linear_distance(&uv1, &uv2);
        println!("Chord distance ≈ {:.2} km", d / 1000.0);
        assert!(d > 100_000.0); // should be ~111 km
    }

    #[test]
    fn test_haversine_distance() {
        let r_um = 6_371_000_000_000; // Earth radius in µm
        let uv1 = encode_uvoxid(r_um, 0, 0);
        let uv2 = encode_uvoxid(r_um, 0, 1_000_000); // 1° east

        let d = haversine_distance(&uv1, &uv2);
        println!("Great-circle distance ≈ {:.2} km", d / 1000.0);
        assert!((d / 1000.0 - 111.0).abs() < 1.0);
    }
}
