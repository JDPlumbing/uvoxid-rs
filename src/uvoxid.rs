// src/uvoxid.rs

/// A UVoxID as 3x64-bit fields, with helpers to pack/unpack into 192-bit form.
/// - r_um: radius in micrometers
/// - lat_microdeg: latitude in millionths of a degree (-90e6..+90e6)
/// - lon_microdeg: longitude in millionths of a degree (-180e6..+180e6)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UvoxId {
    pub r_um: u64,
    pub lat_microdeg: i64,
    pub lon_microdeg: i64,
}

impl UvoxId {
    /// Construct directly from tuple.
    pub fn new(r_um: u64, lat_microdeg: i64, lon_microdeg: i64) -> Self {
        Self { r_um, lat_microdeg, lon_microdeg }
    }

    /// Convert to packed (192-bit) representation.
    pub fn to_packed(&self) -> (u128, u64) {
        let lat_enc = (self.lat_microdeg + 90_000_000) as u64;
        let lon_enc = (self.lon_microdeg + 180_000_000) as u64;

        let high: u128 = (self.r_um as u128) << 64 | (lat_enc as u128);
        let low: u64 = lon_enc;

        (high, low)
    }

    /// Reconstruct from packed (192-bit) representation.
    pub fn from_packed(high: u128, low: u64) -> Self {
        let r_um = (high >> 64) as u64;
        let lat_enc = (high & ((1u128 << 64) - 1)) as u64;
        let lon_enc = low;

        let lat_microdeg = lat_enc as i64 - 90_000_000;
        let lon_microdeg = lon_enc as i64 - 180_000_000;

        Self { r_um, lat_microdeg, lon_microdeg }
    }

    /// Convert to hex string.
    pub fn to_hex(&self) -> String {
        let (high, low) = self.to_packed();
        format!("{:032x}{:016x}", high, low)
    }

    /// Parse from hex string.
    pub fn from_hex(s: &str) -> Option<Self> {
        if s.len() != 48 {
            return None;
        }
        let high = u128::from_str_radix(&s[0..32], 16).ok()?;
        let low = u64::from_str_radix(&s[32..48], 16).ok()?;
        Some(Self::from_packed(high, low))
    }

    /// Return as tuple for easy math.
    pub fn as_tuple(&self) -> (u64, i64, i64) {
        (self.r_um, self.lat_microdeg, self.lon_microdeg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_math_and_pack() {
        let original = UvoxId::new(1_000_000u64, 123_456i64, -654_321i64);

        // Tuple view
        let tup = original.as_tuple();
        assert_eq!(tup, (1_000_000, 123_456, -654_321));

        // Packed round trip
        let (high, low) = original.to_packed();
        let unpacked = UvoxId::from_packed(high, low);
        assert_eq!(original, unpacked);

        // Hex round trip
        let hex = original.to_hex();
        let from_hex = UvoxId::from_hex(&hex).unwrap();
        assert_eq!(original, from_hex);
    }
}
