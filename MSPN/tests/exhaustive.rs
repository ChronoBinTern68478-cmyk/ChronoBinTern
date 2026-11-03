// SPDX-License-Identifier: AGPL-3.0-or-later

use mspn::{mspn_eval, Micro};

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

#[test]
fn exhaustive_n4() {
    let n: usize = 4;
    let max = 2i64.pow(n as u32) - 1;
    for v in -max..=max {
        let (bits, steps) = canonical(v, n);
        assert_eq!(mspn_eval(&bits, &steps), Ok(v), "failed for V = {}", v);
    }
}

#[test]
fn exhaustive_n8() {
    let n: usize = 8;
    let max = 2i64.pow(n as u32) - 1;
    for v in -max..=max {
        let (bits, steps) = canonical(v, n);
        assert_eq!(mspn_eval(&bits, &steps), Ok(v), "failed for V = {}", v);
    }
}