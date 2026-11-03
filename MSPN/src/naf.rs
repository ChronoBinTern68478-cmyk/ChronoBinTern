// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Compute the Non-Adjacent Form (NAF) of an integer.
///
/// NAF uses digits `{−1, 0, +1}` with no two adjacent non-zero
/// digits. It minimizes the number of non-zero digits.
///
/// # Example
///
/// ```
/// use mspn::naf::naf;
///
/// assert_eq!(naf(7), vec![1, 0, 0, -1]);  // 8 − 1 = 7
/// assert_eq!(naf(3), vec![1, 0, -1]);     // 4 − 1 = 3
/// assert_eq!(naf(0), Vec::<i8>::new());
/// ```
#[cfg(feature = "alloc")]
pub fn naf(mut v: i64) -> Vec<i8> {
    if v == 0 {
        return Vec::new();
    }

    let negative = v < 0;
    if negative {
        v = -v;
    }

    let mut digits = Vec::new();
    while v > 0 {
        if v & 1 == 1 {
            let d = 2 - (v % 4) as i8;
            digits.push(d);
            v -= d as i64;
        } else {
            digits.push(0);
        }
        v >>= 1;
    }

    if negative {
        for d in &mut digits {
            *d = -*d;
        }
    }

    digits
}

/// Convert an NAF digit sequence to MSPN (bits, steps).
///
/// # Example
///
/// ```
/// use mspn::naf::naf_to_mspn;
/// use mspn::Micro;
///
/// let (bits, steps) = naf_to_mspn(&[1, 0, 0, -1]);
/// assert_eq!(bits, vec![1, 0, 0, 1]);
/// assert_eq!(steps, vec![Micro::F, Micro::P, Micro::P, Micro::R]);
/// ```
#[cfg(feature = "alloc")]
pub fn naf_to_mspn(digits: &[i8]) -> (Vec<u8>, Vec<Micro>) {
    use crate::Micro;
    let mut bits = Vec::with_capacity(digits.len());
    let mut steps = Vec::with_capacity(digits.len());
    for &d in digits {
        match d {
            1 => {
                bits.push(1);
                steps.push(Micro::F);
            }
            -1 => {
                bits.push(1);
                steps.push(Micro::R);
            }
            0 => {
                bits.push(0);
                steps.push(Micro::P);
            }
            _ => panic!("NAF digit must be in {{-1, 0, 1}}, got {}", d),
        }
    }
    (bits, steps)
}