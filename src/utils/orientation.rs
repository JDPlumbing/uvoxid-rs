// src/utils/orientation.rs
//! Orientation utilities for UVoxID.

use num_bigint::BigUint;
use crate::decode_uvoxid;

/// Compute differences between two UVoxID positions.
///
/// Returns a tuple:
/// - dr_um: radial change in micrometers
/// - dlat_deg: latitude delta in degrees
/// - dlon_deg: longitude delta in degrees (normalized to [-180, 180])
pub fn spherical_delta(uv1: &BigUint, uv2: &BigUint) -> (i64, f64, f64) {
    let (r1, lat1_micro, lon1_micro) = decode_uvoxid(uv1);
    let (r2, lat2_micro, lon2_micro) = decode_uvoxid(uv2);

    let dr_um = r2 as i64 - r1 as i64;
    let dlat_deg = (lat2_micro - lat1_micro) as f64 / 1e6;
    let mut dlon_deg = (lon2_micro - lon1_micro) as f64 / 1e6;

    // Normalize longitude delta into [-180, 180]
    if dlon_deg > 180.0 {
        dlon_deg -= 360.0;
    } else if dlon_deg < -180.0 {
        dlon_deg += 360.0;
    }

    (dr_um, dlat_deg, dlon_deg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{encode_uvoxid};  // ✅ bring encode_uvoxid into scope

    #[test]
    fn test_spherical_delta() {
        let earth_radius_um: u64 = 6_371_000_000_000;

        // Miami vs. NYC
        let miami = encode_uvoxid(earth_radius_um, (25.76 * 1e6) as i64, (-80.19 * 1e6) as i64);
        let nyc   = encode_uvoxid(earth_radius_um, (40.71 * 1e6) as i64, (-74.01 * 1e6) as i64);

        // ✅ pass by reference
        let (dr_um, dlat_deg, dlon_deg) = spherical_delta(&miami, &nyc);

        assert_eq!(dr_um, 0);
        assert!((dlat_deg - 14.95).abs() < 0.1);
        assert!((dlon_deg - 6.18).abs() < 0.1);
    }   
}