// SPDX-License-Identifier: AGPL-3.0-or-later
//! MSPN — Microstep-Signed Positional Notation
//!
//! A reversible forward-backward encoding of Binary Signed Digit (BSD)
//! arithmetic for hybrid Boolean–balanced-ternary systems.
//!
//! # Example
//!
//! ```
//! use mspn::{mspn_eval, Micro};
//!
//! let bits = [1, 0, 1, 1];
//! let steps = [Micro::R, Micro::P, Micro::R, Micro::R];
//! assert_eq!(mspn_eval(&bits, &steps), Ok(-11));
//! ```
//!
//! # Prior art
//!
//! MSPN is mathematically equivalent to BSD (Avizienis, 1961).
//! See `docs/03-prior-art.md` for details.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod canonical;
pub mod error;
pub mod eval;
pub mod micro;
pub mod naf;

pub use error::MspnError;
pub use eval::mspn_eval;
pub use micro::Micro;

/// Width of the accumulator required for `n`-bit input.
///
/// The range is `[−(2ⁿ−1), +(2ⁿ−1)]`, so `n+1` signed bits
/// are always sufficient.
pub const fn acc_width(n: usize) -> usize {
    n + 1
}

/// Scalar dot product for BitNet-style inference.
///
/// Unlike `mspn_eval`, this does **not** shift the accumulator.
/// Each position contributes `s(m) × b` to a running sum.
///
/// # Example
///
/// ```
/// use mspn::{mspn_dot_product, Micro};
///
/// let inputs = [1, 0, 1];
/// let steps  = [Micro::F, Micro::R, Micro::P];
/// assert_eq!(mspn_dot_product(&inputs, &steps), 1);
/// // (+1)(1) + (−1)(0) + (0)(1) = 1
/// ```
pub fn mspn_dot_product(bits: &[u8], steps: &[Micro]) -> i64 {
    let mut acc: i64 = 0;
    for (&b, &m) in bits.iter().zip(steps) {
        acc += m.sign() as i64 * b as i64;
    }
    acc
}


/// Width of the accumulator required for `n`-input scalar
/// (dot product).
///
/// The range is `[−n, +n]`, so `ceil(log₂(n+1)) + 1` signed
/// bits are sufficient.
pub const fn dot_acc_width(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        (usize::BITS - n.leading_zeros()) as usize + 1
    }
}