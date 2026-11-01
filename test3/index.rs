#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MicroStep {
    F,
    R,
    P,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Binary,
    Ternary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RequestKey {
    Nibble(i8),
    Trits((i8, i8, i8)),
}

fn execute_forward() {
    println!("  → F (Forward)");
}

fn execute_reverse() {
    println!("  ← R (Reverse)");
}

fn execute_pause() {
    println!("  ■ P (Pause - чакане)");
}

fn run_instruction(steps: &[MicroStep; 7]) {
    for step in steps {
        match step {
            MicroStep::F => execute_forward(),
            MicroStep::R => execute_reverse(),
            MicroStep::P => execute_pause(),
        }
    }
}

const LUT: [[MicroStep; 7]; 27] = [
    [MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::R, MicroStep::R, MicroStep::R],
    [MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::P],
    [MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::R, MicroStep::R, MicroStep::P, MicroStep::P],
    [MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::F],
    [MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::F, MicroStep::R],
    [MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::F],
    [MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::F, MicroStep::R],
    [MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::P, MicroStep::R, MicroStep::F, MicroStep::P],
    [MicroStep::P, MicroStep::P, MicroStep::R, MicroStep::R, MicroStep::R, MicroStep::F, MicroStep::F],
    [MicroStep::P, MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::P, MicroStep::R, MicroStep::R],
    [MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::R, MicroStep::P, MicroStep::R, MicroStep::P],
    [MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::R, MicroStep::F],
    [MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::R],
    [MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::P],
    [MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::F],
    [MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::R],
    [MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::P],
    [MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::F],
    [MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::R, MicroStep::R],
    [MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::R, MicroStep::P],
    [MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::R, MicroStep::F],
    [MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::R],
    [MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::P, MicroStep::P],
    [MicroStep::F, MicroStep::F, MicroStep::P, MicroStep::P, MicroStep::F, MicroStep::P, MicroStep::F],
    [MicroStep::F, MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::R],
    [MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::P, MicroStep::F, MicroStep::F, MicroStep::P],
    [MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::F, MicroStep::F],
];

fn get_instruction_by_nibble(nibble: i8) -> Option<&'static [MicroStep; 7]> {
    if !(-11..=15).contains(&nibble) {
        return None;
    }

    Some(&LUT[(nibble + 11) as usize])
}

fn get_instruction_by_trits(trits: (i8, i8, i8)) -> Option<&'static [MicroStep; 7]> {
    let (a, b, c) = trits;
    if !(-1..=1).contains(&a) || !(-1..=1).contains(&b) || !(-1..=1).contains(&c) {
        return None;
    }

    let index = ((a + 1) * 9 + (b + 1) * 3 + (c + 1)) as usize;
    LUT.get(index)
}

fn execute_request(mode: Mode, key: RequestKey) {
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

fn main() {
    execute_request(Mode::Binary, RequestKey::Nibble(11));
}
