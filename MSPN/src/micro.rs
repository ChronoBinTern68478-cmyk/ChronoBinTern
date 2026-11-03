// SPDX-License-Identifier: AGPL-3.0-or-later

/// A microstep in an MSPN stream.
///
/// | Microstep | Bit | Sign | μPC action |
/// |-----------|-----|------|------------|
/// | `R` | 1 | −1 | μPC − 1 |
/// | `F` | 1 | +1 | μPC + 1 |
/// | `P` | 0 |  0 | pause 1 tick, then μPC + 1 |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Micro {
    /// Reverse: read backward, subtract the bit.
    R = 0b10,
    /// Forward: read forward, add the bit.
    F = 0b01,
    /// Pause: no contribution, one tick delay.
    P = 0b00,
}

impl Micro {
    /// Sign contribution: `−1` for `R`, `+1` for `F`, `0` for `P`.
    #[inline]
    pub const fn sign(self) -> i8 {
        match self {
            Micro::R => -1,
            Micro::F => 1,
            Micro::P => 0,
        }
    }

    /// Decode from a 2-bit code (as used in the hardware schematic).
    ///
    /// Returns `None` for the reserved code `0b11`.
    #[inline]
    pub const fn from_bits(code: u8) -> Option<Self> {
        match code & 0b11 {
            0b00 => Some(Micro::P),
            0b01 => Some(Micro::F),
            0b10 => Some(Micro::R),
            _ => None,
        }
    }

    /// Encode to a 2-bit code.
    #[inline]
    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}