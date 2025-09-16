// src/utils/geometry.rs
//! Geometry utilities for UVoxID.
//! Pure voxel math: no materials, no mass, just counts and volumes.

const VOXEL_SIZE_M: f64 = 1e-6;
const VOXEL_VOLUME_M3: f64 = VOXEL_SIZE_M * VOXEL_SIZE_M * VOXEL_SIZE_M; // instead of powi(3)


/// Return the volume of a single voxel in cubic meters.
pub fn voxel_volume_m3() -> f64 {
    VOXEL_VOLUME_M3
}

/// Return number of voxels in a cube of given side length (meters).
pub fn cube_voxels(side_m: f64) -> u64 {
    ((side_m / VOXEL_SIZE_M).powi(3)) as u64
}

/// Return number of voxels in a sphere of given radius (meters).
pub fn sphere_voxels(radius_m: f64) -> u64 {
    let volume_m3 = (4.0 / 3.0) * std::f64::consts::PI * radius_m.powi(3);
    (volume_m3 / VOXEL_VOLUME_M3) as u64
}

/// Return number of voxels in a cylinder.
pub fn cylinder_voxels(radius_m: f64, height_m: f64) -> u64 {
    let volume_m3 = std::f64::consts::PI * radius_m.powi(2) * height_m;
    (volume_m3 / VOXEL_VOLUME_M3) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voxel_volume() {
        assert!((voxel_volume_m3() - 1e-18).abs() < 1e-24);
    }

    #[test]
    fn test_cube_voxels() {
        let side_m = 1e-3; // 1 mm
        let voxels = cube_voxels(side_m);
        assert!(voxels > 0);
    }

    #[test]
    fn test_sphere_voxels() {
        let radius_m = 1e-3; // 1 mm
        let voxels = sphere_voxels(radius_m);
        assert!(voxels > 0);
    }

    #[test]
    fn test_cylinder_voxels() {
        let radius_m = 1e-3;
        let height_m = 1e-3;
        let voxels = cylinder_voxels(radius_m, height_m);
        assert!(voxels > 0);
    }
}
