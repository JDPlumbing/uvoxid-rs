// src/uvoxid.rs

/// A 192-bit UVoxID packed as (r_um, lat_microdeg, lon_microdeg).
/// - r_um: radius in micrometers
/// - lat_microdeg: latitude in millionths of a degree (-90e6..+90e6)
/// - lon_microdeg: longitude in millionths of a degree (-180e6..+180e6)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UvoxId(u128, u64); 
// Store as 128+64 = 192 bits

impl UvoxId {
    /// Encode spherical coordinates into UVoxID.
    pub fn encode(r_um: u64, lat_microdeg: i64, lon_microdeg: i64) -> Self {
        let lat_enc = (lat_microdeg + 90_000_000) as u64;
        let lon_enc = (lon_microdeg + 180_000_000) as u64;

        // Pack: [ r (64b) | lat (64b) | lon (64b) ]
        let high: u128 = (r_um as u128) << 64 | (lat_enc as u128);
        let low: u64 = lon_enc;

        Self(high, low)
    }

    /// Decode UVoxID back into spherical coordinates.
    pub fn decode(&self) -> (u64, i64, i64) {
        let r_um = (self.0 >> 64) as u64;
        let lat_enc = (self.0 & ((1u128 << 64) - 1)) as u64;
        let lon_enc = self.1;

        let lat_microdeg = lat_enc as i64 - 90_000_000;
        let lon_microdeg = lon_enc as i64 - 180_000_000;

        (r_um, lat_microdeg, lon_microdeg)
    }

    /// Return as hex string for storage/logging.
    pub fn to_hex(&self) -> String {
        format!("{:032x}{:016x}", self.0, self.1)
    }

    /// Construct from hex string.
    pub fn from_hex(s: &str) -> Option<Self> {
        if s.len() != 48 {
            return None;
        }
        let high = u128::from_str_radix(&s[0..32], 16).ok()?;
        let low = u64::from_str_radix(&s[32..48], 16).ok()?;
        Some(Self(high, low))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let original = (1_000_000u64, 123_456i64, -654_321i64);
        let uvox = UvoxId::encode(original.0, original.1, original.2);
        let decoded = uvox.decode();
        assert_eq!(original, decoded);
    }
}
