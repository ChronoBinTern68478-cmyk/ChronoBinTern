// SPDX-License-Identifier: AGPL-3.0-or-later

use mspn::naf::{naf, naf_to_mspn};
use mspn::mspn_eval;

fn main() {
    for v in [3, 7, 15, -3, -7, -15] {
        let digits = naf(v);
        let (bits, steps) = naf_to_mspn(&digits);
        let result = mspn_eval(&bits, &steps).unwrap();
        println!(
            "V = {:>3}  NAF = {:?}  MSPN = {:?}  →  {}",
            v, digits, steps, result
        );
    }
}