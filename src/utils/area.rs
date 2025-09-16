// src/utils/area.rs
//! Area utilities for UVoxID.
//!
//! These functions work with decoded UVoxIDs (r, lat, lon)
//! to compute surface areas on spheres and planar patches.

use std::f64::consts::PI;

use crate::decode_uvoxid; // assumes `decode_uvoxid` lives in crate root

/// Compute the area (in m²) of a spherical quadrilateral patch on a sphere
/// bounded by latitude/longitude coordinates.
///
/// # Arguments
/// - `r_um`: radius in micrometers
/// - `lat1_deg`, `lat2_deg`: latitude bounds in degrees
/// - `lon1_deg`, `lon2_deg`: longitude bounds in degrees
///
/// # Returns
/// Area in square meters
pub fn spherical_patch_area(
    r_um: u64,
    lat1_deg: f64,
    lat2_deg: f64,
    lon1_deg: f64,
    lon2_deg: f64,
) -> f64 {
    // Convert radius to meters
    let r_m = r_um as f64 * 1e-6;

    // Convert to radians
    let lat1 = lat1_deg.to_radians();
    let lat2 = lat2_deg.to_radians();
    let lon1 = lon1_deg.to_radians();
    let lon2 = lon2_deg.to_radians();

    // Spherical patch area formula:
    // A = R² * Δλ * (sin φ2 − sin φ1)
    let delta_lon = (lon2 - lon1).abs();
    (r_m.powi(2)) * delta_lon * (lat2.sin() - lat1.sin()).abs()
}

/// Approximate area (in m²) spanned between two UVoxIDs on the same sphere.
/// Useful for bounding box checks.
///
/// # Arguments
/// - `uv1`, `uv2`: UVoxID integers
///
/// # Returns
/// Surface patch area in square meters
pub fn area_between_voxels(uv1: &num_bigint::BigUint, uv2: &num_bigint::BigUint) -> f64 {
    let (r1, lat1_micro, lon1_micro) = decode_uvoxid(uv1);
    let (r2, lat2_micro, lon2_micro) = decode_uvoxid(uv2);

    if r1 != r2 {
        panic!("Both voxels must be on the same spherical shell (same radius).");
    }

    let lat1 = lat1_micro as f64 / 1e6;
    let lat2 = lat2_micro as f64 / 1e6;
    let lon1 = lon1_micro as f64 / 1e6;
    let lon2 = lon2_micro as f64 / 1e6;

    spherical_patch_area(r1, lat1, lat2, lon1, lon2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_area_equator() {
        // Earth radius in micrometers
        let earth_radius_um: u64 = 6_371_000_000_000;
        let area = spherical_patch_area(earth_radius_um, 0.0, 1.0, 0.0, 1.0);
        println!("1°x1° patch at equator ≈ {:.2e} m²", area);
        assert!(area > 1.0e10); // rough check
    }
}
