// src/utils/tolerance.rs
//! Tolerance utilities for UVoxID.

use num_bigint::BigUint;
use crate::{encode_uvoxid, decode_uvoxid};

const TOTAL_BITS: usize = 192;
const BITS_PER_CHAR: usize = 5;

/// Truncate a UVoxID integer to a given number of significant characters.
pub fn truncate_to_tolerance(uvoxid: &BigUint, sig_chars: usize) -> BigUint {
    let keep_bits = sig_chars * BITS_PER_CHAR;
    if keep_bits > TOTAL_BITS {
        panic!("sig_chars too large, max is {}", TOTAL_BITS / BITS_PER_CHAR);
    }

    let mask = (&BigUint::from(1u128) << keep_bits) - 1u32;
    let mask = mask << (TOTAL_BITS - keep_bits);
    uvoxid & mask
}

/// Check if two UVoxIDs are equal within a given tolerance.
pub fn equal_within_tolerance(a: &BigUint, b: &BigUint, sig_chars: usize) -> bool {
    truncate_to_tolerance(a, sig_chars) == truncate_to_tolerance(b, sig_chars)
}

/// Snap a UVoxID into hex string form at given tolerance.
pub fn snap_to_tolerance(uvoxid: &BigUint, sig_chars: usize) -> String {
    let truncated = truncate_to_tolerance(uvoxid, sig_chars);
    let hex = format!("{:048x}", truncated); // 192 bits = 48 hex chars
    let prefix = &hex[..(sig_chars * BITS_PER_CHAR + 3) / 4]; // round up to full hex digit
    format!("uvoxid:{}{}", prefix, "0".repeat(48 - prefix.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_and_equal() {
        let r = 6_371_000_000_000u64;
        let a = encode_uvoxid(r, (25.76 * 1e6) as i64, (-80.19 * 1e6) as i64);
        let b = encode_uvoxid(r, (25.7601 * 1e6) as i64, (-80.1901 * 1e6) as i64);

        assert!(equal_within_tolerance(&a, &b, 6)); // tolerant
        assert!(!equal_within_tolerance(&a, &b, 32)); // very strict
    }

    #[test]
    fn test_snap() {
        let r = 6_371_000_000_000u64;
        let uv = encode_uvoxid(r, (25.76 * 1e6) as i64, (-80.19 * 1e6) as i64);

        let snap6 = snap_to_tolerance(&uv, 6);
        assert!(snap6.starts_with("uvoxid:"));
        assert_eq!(snap6.len(), "uvoxid:".len() + 48);
    }
}
