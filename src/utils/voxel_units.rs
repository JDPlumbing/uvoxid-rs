// src/utils/voxel_units.rs
//! Voxel unit conversion utilities for UVoxID.
//!
//! 1 voxel = 1 µm³ volume (1 µm edge length).

use std::collections::HashMap;

pub const VOXEL_SIZE_M: f64 = 1e-6;

/// Conversion factors to meters.
pub fn unit_to_meters() -> HashMap<&'static str, f64> {
    HashMap::from([
        ("um", 1e-6),
        ("mm", 1e-3),
        ("cm", 1e-2),
        ("m", 1.0),
        ("km", 1e3),
        ("in", 0.0254),
        ("ft", 0.3048),
        ("yd", 0.9144),
        ("mi", 1609.34),
    ])
}

/// Convert a physical length into voxel count (1 voxel = 1 µm).
pub fn to_voxels(value: f64, unit: &str) -> Result<i64, String> {
    let units = unit_to_meters();
    match units.get(&unit.to_lowercase().as_str()) {
        Some(factor) => {
            let meters = value * factor;
            Ok((meters / VOXEL_SIZE_M).round() as i64)
        }
        None => Err(format!("Unsupported unit: {}", unit)),
    }
}

/// Convert a voxel count back into the requested unit.
pub fn from_voxels(voxels: i64, unit: &str) -> Result<f64, String> {
    let units = unit_to_meters();
    match units.get(&unit.to_lowercase().as_str()) {
        Some(factor) => {
            let meters = voxels as f64 * VOXEL_SIZE_M;
            Ok(meters / factor)
        }
        None => Err(format!("Unsupported unit: {}", unit)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_um_to_voxels() {
        assert_eq!(to_voxels(1.0, "um").unwrap(), 1);
    }

    #[test]
    fn test_mm_to_voxels() {
        assert_eq!(to_voxels(1.0, "mm").unwrap(), 1000);
    }

    #[test]
    fn test_voxels_to_mm() {
        assert_eq!(from_voxels(1000, "mm").unwrap(), 1.0);
    }

    #[test]
    fn test_invalid_unit() {
        assert!(to_voxels(1.0, "parsecs").is_err());
    }
}
