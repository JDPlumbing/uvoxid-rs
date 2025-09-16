// src/utils/tolerance.rs
//! Tolerance utilities for UVoxID.
//!
//! In UVoxID, tolerances are applied by truncating the integer
//! to a certain number of significant bits, corresponding to
//! Base32 characters in the original design.

use num_bigint::BigUint;

/// Total bits in a UVoxID (fixed at 192).
const TOTAL_BITS: usize = 192;
/// Each Base32 char corresponds to 5 bits.
const BITS_PER_CHAR: usize = 5;
/// Max significant characters.
const MAX_SIG_CHARS: usize = TOTAL_BITS / BITS_PER_CHAR;

/// Truncate a UVoxID integer to a given number of significant Base32 characters.
pub fn truncate_to_tolerance(uvoxid: &BigUint, sig_chars: usize) -> BigUint {
    if sig_chars > MAX_SIG_CHARS {
        panic!("sig_chars too large, max is {}", MAX_SIG_CHARS);
    }
    let keep_bits = sig_chars * BITS_PER_CHAR;
    let drop_bits = TOTAL_BITS - keep_bits;

    // Build mask that keeps the *high bits* and zeroes out the low ones
    let mask = ((BigUint::from(1u128) << TOTAL_BITS) - 1u32) << drop_bits;
    uvoxid & mask
}


/// Check if two UVoxIDs are equal within a given tolerance.
pub fn equal_within_tolerance(a: &BigUint, b: &BigUint, sig_chars: usize) -> bool {
    truncate_to_tolerance(a, sig_chars) == truncate_to_tolerance(b, sig_chars)
}

/// Snap a UVoxID to hex string at a given tolerance.
pub fn snap_to_tolerance(uvoxid: &BigUint, sig_chars: usize) -> String {
    let truncated = truncate_to_tolerance(uvoxid, sig_chars);
    format!("uvoxid:{:048x}", truncated) // 192 bits = 48 hex chars
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode_uvoxid;

    #[test]
    fn test_truncate_and_equal() {
        let r = 6_371_000_000_000u64;
        let a = encode_uvoxid(r, (25.76 * 1e6) as i64, (-80.19 * 1e6) as i64);
        let b = encode_uvoxid(r, (25.7601 * 1e6) as i64, (-80.1901 * 1e6) as i64);

        assert!(!equal_within_tolerance(&a, &b, 38)); // strict, they differ
        assert!(equal_within_tolerance(&a, &b, 5));   // coarse, they match
    }


    #[test]
    fn test_snap() {
        let r = 6_371_000_000_000u64;
        let uv = encode_uvoxid(r, (25.76 * 1e6) as i64, (-80.19 * 1e6) as i64);

        let snapped = snap_to_tolerance(&uv, 8);
        assert!(snapped.starts_with("uvoxid:"));
        assert_eq!(snapped.len(), "uvoxid:".len() + 48); // 48 hex chars
    }
}
