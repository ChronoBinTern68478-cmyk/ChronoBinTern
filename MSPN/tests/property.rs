// SPDX-License-Identifier: AGPL-3.0-or-later

use mspn::{mspn_eval, Micro};
use proptest::prelude::*;

fn canonical(v: i64, n: usize) -> (Vec<u8>, Vec<Micro>) {
    let abs = v.unsigned_abs();
    let bits: Vec<u8> = (0..n)
        .rev()
        .map(|i| ((abs >> i) & 1) as u8)
        .collect();
    let steps: Vec<Micro> = bits
        .iter()
        .map(|&b| {
            if b == 0 {
                Micro::P
            } else if v >= 0 {
                Micro::F
            } else {
                Micro::R
            }
        })
        .collect();
    (bits, steps)
}

proptest! {
    #[test]
    fn roundtrip_n4(v in -15i64..=15) {
        let (bits, steps) = canonical(v, 4);
        prop_assert_eq!(mspn_eval(&bits, &steps), Ok(v));
    }

    #[test]
    fn roundtrip_n8(v in -255i64..=255) {
        let (bits, steps) = canonical(v, 8);
        prop_assert_eq!(mspn_eval(&bits, &steps), Ok(v));
    }

    #[test]
    fn symmetry(v in 0i64..=255) {
        let (bits_pos, steps_pos) = canonical(v, 8);
        let (bits_neg, steps_neg) = canonical(-v, 8);
        prop_assert_eq!(mspn_eval(&bits_pos, &steps_pos), Ok(v));
        prop_assert_eq!(mspn_eval(&bits_neg, &steps_neg), Ok(-v));
    }
}