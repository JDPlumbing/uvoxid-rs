// src/delta.rs
use std::ops::{Add, Sub};

use crate::uvoxid::UvoxId;

/// A vector difference in spherical coordinates (Δr, Δlat, Δlon).
/// Uses i128 internally so we never overflow when subtracting large u64 values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Delta {
    pub dr: i128,
    pub dlat: i128,
    pub dlon: i128,
}

impl Delta {
    pub fn new(dr: i128, dlat: i128, dlon: i128) -> Self {
        Self { dr, dlat, dlon }
    }

    /// Scale a delta by a factor (useful for projecting positions forward).
    pub fn scale(&self, factor: i128) -> Self {
        Self {
            dr: self.dr * factor,
            dlat: self.dlat * factor,
            dlon: self.dlon * factor,
        }
    }
}

/// Subtract two UvoxIds to get a Delta
impl Sub<UvoxId> for UvoxId {
    type Output = Delta;

    fn sub(self, other: UvoxId) -> Delta {
        Delta {
            dr: self.r_um as i128 - other.r_um as i128,
            dlat: self.lat_microdeg as i128 - other.lat_microdeg as i128,
            dlon: self.lon_microdeg as i128 - other.lon_microdeg as i128,
        }
    }
}

/// Add a Delta to a UvoxId to get a new UvoxId
impl Add<Delta> for UvoxId {
    type Output = UvoxId;

    fn add(self, delta: Delta) -> UvoxId {
        UvoxId {
            r_um: (self.r_um as i128 + delta.dr) as u64,
            lat_microdeg: (self.lat_microdeg as i128 + delta.dlat) as i64,
            lon_microdeg: (self.lon_microdeg as i128 + delta.dlon) as i64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uvoxid::UvoxId;

    #[test]
    fn delta_round_trip() {
        let id1 = UvoxId::new(1_000_000, 100, 200);
        let id2 = UvoxId::new(1_000_500, 120, 180);

        let delta = id2 - id1;
        assert_eq!(delta, Delta::new(500, 20, -20));

        let id3 = id1 + delta;
        assert_eq!(id2, id3);
    }
}
