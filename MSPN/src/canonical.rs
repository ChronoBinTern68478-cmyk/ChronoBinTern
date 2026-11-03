// SPDX-License-Identifier: AGPL-3.0-or-later
//! Canonical MSPN encodings.
//!
//! The canonical (trivial) encoding from `02-proof.md`:
//!
//! - For `V >= 0`: `f` for 1-bits, `p` for 0-bits.
//! - For `V < 0`:  `r` for 1-bits, `p` for 0-bits.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
use crate::error::MspnError;

#[cfg(feature = "alloc")]
use crate::micro::Micro;

/// Generate the canonical MSPN encoding for `v` using `n` positions.
///
/// Requires the `alloc` feature.
#[cfg(feature = "alloc")]
pub fn canonical_encode(v: i64, n: usize) -> Result<(Vec<u8>, Vec<Micro>), MspnError> {
    let max = if n >= 63 {
        i64::MAX
    } else {
        (1i64 << n) - 1
    };

    if v > max || v < -max {
        return Err(MspnError::Overflow);
    }

    let abs = v.unsigned_abs();

    let mut bits = Vec::with_capacity(n);
    let mut steps = Vec::with_capacity(n);

    for i in (0..n).rev() {
        let bit = ((abs >> i) & 1) as u8;
        bits.push(bit);

        let step = if bit == 0 {
            Micro::P
        } else if v >= 0 {
            Micro::F
        } else {
            Micro::R
        };
        steps.push(step);
    }

    Ok((bits, steps))
}

/// Minimum number of positions required to encode `v`.
///
/// This function does **not** require `alloc`.
pub fn canonical_width(v: i64) -> usize {
    let abs = v.unsigned_abs();
    if abs == 0 {
        1
    } else {
        (64 - abs.leading_zeros()) as usize
    }
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use super::*;

    #[test]
    fn encode_zero() {
        let (bits, steps) = canonical_encode(0, 4).unwrap();
        assert_eq!(bits, vec![0, 0, 0, 0]);
        assert_eq!(steps, vec![Micro::P; 4]);
    }

    #[test]
    fn encode_positive() {
        let (bits, steps) = canonical_encode(11, 4).unwrap();
        assert_eq!(bits, vec![1, 0, 1, 1]);
        assert_eq!(
            steps,
            vec![Micro::F, Micro::P, Micro::F, Micro::F]
        );
    }

    #[test]
    fn encode_negative() {
        let (bits, steps) = canonical_encode(-11, 4).unwrap();
        assert_eq!(bits, vec![1, 0, 1, 1]);
        assert_eq!(
            steps,
            vec![Micro::R, Micro::P, Micro::R, Micro::R]
        );
    }

    #[test]
    fn overflow() {
        assert!(canonical_encode(16, 4).is_err());
        assert!(canonical_encode(-16, 4).is_err());
    }

    #[test]
    fn width() {
        assert_eq!(canonical_width(0), 1);
        assert_eq!(canonical_width(1), 1);
        assert_eq!(canonical_width(15), 4);
        assert_eq!(canonical_width(16), 5);
    }
}