// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
use crate::micro::Micro;

/// Compute the Non-Adjacent Form (NAF) of an integer.
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
#[cfg(feature = "alloc")]
pub fn naf_to_mspn(digits: &[i8]) -> (Vec<u8>, Vec<Micro>) {
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