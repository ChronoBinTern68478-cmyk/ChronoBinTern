// SPDX-License-Identifier: AGPL-3.0-or-later

use mspn::{mspn_eval, Micro};

fn main() {
    // 1011 with r p r r → −11
    let bits = [1, 0, 1, 1];
    let steps = [Micro::R, Micro::P, Micro::R, Micro::R];
    println!("1011 with r p r r → {:?}", mspn_eval(&bits, &steps));

    // 1011 with f p f f → +11
    let steps = [Micro::F, Micro::P, Micro::F, Micro::F];
    println!("1011 with f p f f → {:?}", mspn_eval(&bits, &steps));
}