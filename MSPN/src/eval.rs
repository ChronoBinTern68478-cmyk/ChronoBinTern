// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::error::MspnError;
use crate::micro::Micro;

// ============================================================
// Positional MSPN (for numbers)
// ============================================================

/// Evaluate a positional MSPN stream.
///
/// Computes:
///
/// ```text
/// V = Σᵢ s(mᵢ) × bᵢ × 2^(n−1−i)
/// ```
///
/// where `s(R) = −1`, `s(F) = +1`, `s(P) = 0`.
///
/// This is the **positional** variant, which enforces the
/// constraint `b = 1 ⟺ m ∈ {R, F}`.
///
/// # Errors
///
/// Returns an error if:
/// - `bits` and `steps` have different lengths
/// - any bit is not 0 or 1
/// - the constraint `b = 1 ⟺ m ∈ {R, F}` is violated
/// - the accumulator overflows `i64`
///
/// # Example
///
/// ```
/// use mspn::{mspn_eval, Micro};
///
/// assert_eq!(
///     mspn_eval(&[1, 0, 1, 1], &[Micro::R, Micro::P, Micro::R, Micro::R]),
///     Ok(-11)
/// );
/// ```
pub fn mspn_eval(bits: &[u8], steps: &[Micro]) -> Result<i64, MspnError> {
    if bits.len() != steps.len() {
        return Err(MspnError::LengthMismatch {
            bits: bits.len(),
            steps: steps.len(),
        });
    }

    let mut acc: i64 = 0;

    for (i, (&b, &m)) in bits.iter().zip(steps).enumerate() {
        if b > 1 {
            return Err(MspnError::InvalidBit {
                position: i,
                value: b,
            });
        }

        let constraint_ok = match m {
            Micro::P => b == 0,
            Micro::R | Micro::F => b == 1,
        };
        if !constraint_ok {
            return Err(MspnError::ConstraintViolation {
                position: i,
                bit: b,
                micro: m.to_bits(),
            });
        }

        acc = acc.checked_shl(1).ok_or(MspnError::Overflow)?;

        match m {
            Micro::R => {
                acc = acc.checked_sub(b as i64).ok_or(MspnError::Overflow)?
            }
            Micro::F => {
                acc = acc.checked_add(b as i64).ok_or(MspnError::Overflow)?
            }
            Micro::P => {}
        }
    }

    Ok(acc)
}

/// Unchecked positional evaluation (faster, no validation).
pub fn mspn_eval_unchecked(bits: &[u8], steps: &[Micro]) -> i64 {
    debug_assert_eq!(bits.len(), steps.len());
    let mut acc: i64 = 0;
    for (&b, &m) in bits.iter().zip(steps) {
        acc = acc.wrapping_shl(1);
        match m {
            Micro::R => acc = acc.wrapping_sub(b as i64),
            Micro::F => acc = acc.wrapping_add(b as i64),
            Micro::P => {}
        }
    }
    acc
}

// ============================================================
// Scalar MSPN (for dot products / neural networks)
// ============================================================

/// Evaluate a scalar MSPN stream (dot product).
///
/// Computes:
///
/// ```text
/// output = Σᵢ s(mᵢ) × bᵢ
/// ```
///
/// Unlike [`mspn_eval`], this does **not** shift the accumulator.
/// Each position contributes `s(m) × b` to a running sum.
///
/// This is the **scalar** variant, which does **not** enforce
/// the positional constraint `b = 1 ⟺ m ∈ {R, F}`. This is
/// necessary because in a neural network, a zero input with a
/// non-zero weight still gives zero contribution.
///
/// # Errors
///
/// Returns an error if:
/// - `bits` and `steps` have different lengths
/// - any bit is not 0 or 1
/// - the accumulator overflows `i64`
///
/// # Example
///
/// ```
/// use mspn::{mspn_dot_product, Micro};
///
/// // BitNet-style neuron:
/// // inputs:  1 0 1 1
/// // weights: +1 −1 0 +1
/// // steps:   f  r  p  f
/// let inputs = [1, 0, 1, 1];
/// let steps  = [Micro::F, Micro::R, Micro::P, Micro::F];
/// assert_eq!(mspn_dot_product(&inputs, &steps), Ok(2));
/// // (+1)(1) + (−1)(0) + (0)(1) + (+1)(1) = 1 + 0 + 0 + 1 = 2
/// ```
pub fn mspn_dot_product(bits: &[u8], steps: &[Micro]) -> Result<i64, MspnError> {
    if bits.len() != steps.len() {
        return Err(MspnError::LengthMismatch {
            bits: bits.len(),
            steps: steps.len(),
        });
    }

    let mut acc: i64 = 0;

    for (i, (&b, &m)) in bits.iter().zip(steps).enumerate() {
        if b > 1 {
            return Err(MspnError::InvalidBit {
                position: i,
                value: b,
            });
        }

        let contribution = m.sign() as i64 * b as i64;
        acc = acc
            .checked_add(contribution)
            .ok_or(MspnError::Overflow)?;
    }

    Ok(acc)
}

/// Unchecked scalar evaluation (faster, no validation).
pub fn mspn_dot_product_unchecked(bits: &[u8], steps: &[Micro]) -> i64 {
    debug_assert_eq!(bits.len(), steps.len());
    let mut acc: i64 = 0;
    for (&b, &m) in bits.iter().zip(steps) {
        acc = acc.wrapping_add(m.sign() as i64 * b as i64);
    }
    acc
}