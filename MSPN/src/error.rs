// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

/// Errors that can occur in MSPN operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MspnError {
    /// The bits and steps slices have different lengths.
    LengthMismatch { bits: usize, steps: usize },

    /// A bit value was not 0 or 1.
    InvalidBit { position: usize, value: u8 },

    /// The constraint `b = 1 ⟺ m ∈ {R, F}` was violated.
    ///
    /// This happens when `b = 0` and `m ∈ {R, F}`, or
    /// when `b = 1` and `m = P`.
    ConstraintViolation {
        position: usize,
        bit: u8,
        micro: u8,
    },

    /// The accumulator overflowed `i64`.
    Overflow,
}

impl fmt::Display for MspnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MspnError::LengthMismatch { bits, steps } => write!(
                f,
                "length mismatch: {} bits, {} steps",
                bits, steps
            ),
            MspnError::InvalidBit { position, value } => write!(
                f,
                "invalid bit at position {}: {} (expected 0 or 1)",
                position, value
            ),
            MspnError::ConstraintViolation {
                position,
                bit,
                micro,
            } => write!(
                f,
                "constraint violation at position {}: bit={}, micro={:#04b}",
                position, bit, micro
            ),
            MspnError::Overflow => write!(f, "accumulator overflow"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MspnError {}