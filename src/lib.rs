#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicroStep {
    F,
    R,
    P,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Binary,
    Ternary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestKey {
    Nibble(i8),
    Trits((i8, i8, i8)),
}

pub fn execute_forward() {
    println!("  → F (Forward)");
}

pub fn execute_reverse() {
    println!("  ← R (Reverse)");
}

pub fn execute_pause() {
    println!("  ■ P (Pause - чакане)");
}

pub fn run_instruction(steps: &[MicroStep; 7]) {
    for step in steps {
        match step {
            MicroStep::F => execute_forward(),
            MicroStep::R => execute_reverse(),
            MicroStep::P => execute_pause(),
        }
    }
}

pub const LUT: [[MicroStep; 7]; 27] = [
    // Index 0: -11, nibble 1011, ternary -1 -1 -1
    [MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::R, MicroStep::R, MicroStep::R],
    // Index 1: -10, nibble 1010, ternary -1 -1 0
    [MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::P],
    // Index 2: -9, nibble 1001, ternary -1 0 0
    [MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::R, MicroStep::R, MicroStep::P, MicroStep::P],
    // Index 3: -8, nibble 1000, ternary -1 0 +1
    [MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::F],
    // Index 4: -7, nibble 0111, ternary -1 +1 -1
    [MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::F, MicroStep::R],
    // Index 5: -6, nibble 0110, ternary -1 0 +1
    [MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::F],
    // Index 6: -5, nibble 0101, ternary -1 +1 -1
    [MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::F, MicroStep::R],
    // Index 7: -4, nibble 0100, ternary -1 +1 0
    [MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::P, MicroStep::R, MicroStep::F, MicroStep::P],
    // Index 8: -3, nibble 0011, ternary -1 +1 +1
    [MicroStep::P, MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::R, MicroStep::F, MicroStep::F],
    // Index 9: -2, nibble 0010, ternary 0 -1 -1
    [MicroStep::P, MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::P, MicroStep::R, MicroStep::R],
    // Index 10: -1, nibble 0001, ternary 0 -1 0
    [MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::P],
    // Index 11: 0, nibble 0000, ternary 0 -1 +1
    [MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::R, MicroStep::F],
    // Index 12: 1, nibble 0001, ternary 0 0 -1
    [MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::R],
    // Index 13: 2, nibble 0010, ternary 0 0 0
    [MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::P],
    // Index 14: 3, nibble 0011, ternary 0 0 +1
    [MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::F],
    // Index 15: 4, nibble 0100, ternary 0 +1 -1
    [MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::R],
    // Index 16: 5, nibble 0101, ternary 0 +1 0
    [MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::P],
    // Index 17: 6, nibble 0110, ternary 0 +1 +1
    [MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::F],
    // Index 18: 7, nibble 0111, ternary +1 -1 -1
    [MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::R, MicroStep::R],
    // Index 19: 8, nibble 1000, ternary +1 -1 0
    [MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::R, MicroStep::P],
    // Index 20: 9, nibble 1001, ternary +1 -1 +1
    [MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::R, MicroStep::F],
    // Index 21: 10, nibble 1010, ternary +1 0 -1
    [MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::R],
    // Index 22: 11, nibble 1011, ternary +1 0 0
    [MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::P, MicroStep::P],
    // Index 23: 12, nibble 1100, ternary +1 0 +1
    [MicroStep::F, MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::F],
    // Index 24: 13, nibble 1101, ternary +1 +1 -1
    [MicroStep::F, MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::R],
    // Index 25: 14, nibble 1110, ternary +1 +1 0
    [MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::P],
    // Index 26: 15, nibble 1111, ternary +1 +1 +1
    [MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::F],
];

pub fn get_instruction_by_nibble(nibble: i8) -> Option<&'static [MicroStep; 7]> {
    if !( -11..=15).contains(&nibble) {
        return None;
    }

    Some(&LUT[(nibble + 11) as usize])
}

pub fn get_instruction_by_trits(trits: (i8, i8, i8)) -> Option<&'static [MicroStep; 7]> {
    let (a, b, c) = trits;
    if !(-1..=1).contains(&a) || !(-1..=1).contains(&b) || !(-1..=1).contains(&c) {
        return None;
    }

    let index = ((a + 1) * 9 + (b + 1) * 3 + (c + 1)) as usize;
    LUT.get(index)
}

pub fn execute_request(mode: Mode, key: RequestKey) {
    let steps = match (mode, key) {
        (Mode::Binary, RequestKey::Nibble(nibble)) => get_instruction_by_nibble(nibble),
        (Mode::Ternary, RequestKey::Trits(trits)) => get_instruction_by_trits(trits),
        _ => panic!("mode/key mismatch: the request does not match the selected encoding"),
    };

    match steps {
        Some(steps) => run_instruction(steps),
        None => panic!("invalid request"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_minus_11() {
        let steps: [MicroStep; 7] = [
            MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::R,
            MicroStep::R, MicroStep::R, MicroStep::R,
        ];
        run_instruction(&steps);
        assert_eq!(get_instruction_by_nibble(-11), Some(&steps));
    }

    #[test]
    fn test_instruction_plus_15() {
        let steps: [MicroStep; 7] = [
            MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::F,
            MicroStep::F, MicroStep::F, MicroStep::F,
        ];
        run_instruction(&steps);
        assert_eq!(get_instruction_by_nibble(15), Some(&steps));
    }

    #[test]
    fn test_lookup_by_ternary_trits() {
        assert_eq!(get_instruction_by_trits((-1, -1, -1)), Some(&LUT[0]));
        assert_eq!(get_instruction_by_trits((1, 1, 1)), Some(&LUT[26]));
        assert_eq!(get_instruction_by_trits((0, 0, 0)), Some(&LUT[13]));
    }

    #[test]
    fn test_execute_request_accepts_valid_modes() {
        execute_request(Mode::Binary, RequestKey::Nibble(11));
        execute_request(Mode::Ternary, RequestKey::Trits((1, 0, 0)));
    }
}
