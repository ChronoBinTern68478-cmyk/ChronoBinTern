# Proof of Completeness for MSPN

## 1. Notation

Let:

- `n ≥ 1` be the number of bit positions.
- `b₀, b₁, ..., bₙ₋₁ ∈ {0, 1}` be the input bit sequence.
- `m₀, m₁, ..., mₙ₋₁ ∈ {r, f, p}` be the microstep sequence.
- `Wᵢ = 2^(n−1−i)` be the positional weight of position `i`.
- `s: {r, f, p} → {−1, 0, +1}` be the sign function:

  ```text
  s(r) = −1
  s(f) = +1
  s(p) =  0
  ```

The MSPN value of a (bits, steps) pair is defined as:

```text
V = Σᵢ s(mᵢ) × bᵢ × Wᵢ
```

with the constraint:

```text
bᵢ = 1  ⟺  mᵢ ∈ {r, f}
bᵢ = 0  ⟺  mᵢ = p
```

## 2. Theorem statement

**Theorem (completeness).** For every `n ≥ 1` and every integer `V` such that:

```text
|V| ≤ 2ⁿ − 1
```

there exist:

- a bit sequence `b₀..bₙ₋₁ ∈ {0,1}ⁿ`
- a microstep sequence `m₀..mₙ₋₁ ∈ {r,f,p}ⁿ`

satisfying the constraint above and such that:

```text
V = Σᵢ s(mᵢ) × bᵢ × 2^(n−1−i)
```

## 3. Proof

### Case 1: `V ≥ 0`

Let B = Bₙ₋₁ Bₙ₋₂ ... B₀ be the standard n-bit unsigned binary
representation of V. It exists because 0 ≤ V ≤ 2ⁿ − 1.

Define:

```text
bᵢ = Bᵢ
mᵢ = f, if bᵢ = 1
mᵢ = p, if bᵢ = 0
```

Then:

```text
Σᵢ s(mᵢ) × bᵢ × 2^(n−1−i)
  = Σᵢ (where bᵢ = 1) (+1) × 2^(n−1−i)
  = Σᵢ Bᵢ × 2^(n−1−i)
  = V
```

### Case 2: `V < 0`

Let W = −V. Then 1 ≤ W ≤ 2ⁿ − 1.
Let B be the standard n-bit representation of W.

Define:

```text
bᵢ = Bᵢ
mᵢ = r, if bᵢ = 1
mᵢ = p, if bᵢ = 0
```

Then:

```text
Σᵢ s(mᵢ) × bᵢ × 2^(n−1−i)
  = Σᵢ (where bᵢ = 1) (−1) × 2^(n−1−i)
  = −Σᵢ Bᵢ × 2^(n−1−i)
  = −W
  = V
```

Therefore, the theorem is proved.

## 4. Corollaries

### 4.1 Range

```text
V ∈ [−(2ⁿ − 1), +(2ⁿ − 1)]
```

The number of representable values is:

```text
2 × (2ⁿ − 1) + 1 = 2^(n+1) − 1
```

### 4.2 Symmetry

The range is fully symmetric around zero.
Unlike two's complement, there is no asymmetry
(−2^(n−1) vs +2^(n−1)−1).

### 4.3 Number of encodings

Each position has 3 possible states:

| State | `mᵢ` | `bᵢ` | Contribution |
|-------|------|------|--------------|
| Zero  | `p`  | `0`  | `0`          |
| Plus  | `f`  | `1`  | `+W`         |
| Minus | `r`  | `1`  | `−W`         |

Total encodings: 3ⁿ.

For n = 4: 3⁴ = 81 encodings for 31 values → redundancy.

### 4.4 Redundancy

MSPN is redundant: a single value may have multiple encodings.

Example for V = 1, n = 2:

```text
[0,1] with [p,f]  →  0 + 1 = 1
[1,1] with [f,r]  →  +2 − 1 = 1
```

Both yield 1. Redundancy is present.

## 5. Range analysis

| `n` | Range            | Values | Encodings |
|-----|------------------|--------|-----------|
| 1   | −1 .. +1         | 3      | 3         |
| 2   | −3 .. +3         | 7      | 9         |
| 4   | −15 .. +15       | 31     | 81        |
| 8   | −255 .. +255     | 511    | 6561      |
| 16  | −65535 .. +65535 | 131071 | 43M       |

## 6. Connection to BSD

MSPN is mathematically equivalent to Binary Signed Digit (BSD)
representation, introduced by Algirdas Avizienis in 1961.

If we define:

```text
dᵢ = s(mᵢ) × bᵢ
```

then dᵢ ∈ {−1, 0, +1} and:

```text
V = Σᵢ dᵢ × 2^(n−1−i)
```

This is exactly BSD.

The proof above therefore establishes two things:

- MSPN is complete — every value in [−(2ⁿ−1), +(2ⁿ−1)] has an encoding.
- MSPN is equivalent to BSD — with dᵢ = s(mᵢ) × bᵢ.

The difference is not in the number representation, but in how
the digit is produced: in BSD the digit is stored, in MSPN it is
derived from a bit and a microstep.

## 7. Edge cases

### 7.1 `n = 1`

Range: −1 .. +1

Encodings: 3¹ = 3

| `b` | `m` | `V` |
|-----|-----|-----|
| 0   | `p` | 0   |
| 1   | `f` | +1  |
| 1   | `r` | −1  |

Full coverage. ✓

### 7.2 `V = 0`

The only encoding is bᵢ = 0 and mᵢ = p for all i.
There is no +0 / −0. ✓

### 7.3 `V = 2ⁿ − 1`

```text
bᵢ = 1 for all i
mᵢ = f for all i
```

### 7.4 `V = −(2ⁿ − 1)`

```text
bᵢ = 1 for all i
mᵢ = r for all i
```

## 8. Implementation verification

### 8.1 Exhaustive test for `n = 4`

```rust
#[test]
fn exhaustive_n4() {
    let n = 4;
    for v in -(2i64.pow(n) - 1)..=(2i64.pow(n) - 1) {
        let abs = v.abs() as u64;
        let bits: Vec<u8> = (0..n).rev()
            .map(|i| ((abs >> i) & 1) as u8)
            .collect();
        let steps: Vec<Micro> = bits.iter()
            .map(|&b| {
                if b == 0 { Micro::P }
                else if v >= 0 { Micro::F }
                else { Micro::R }
            })
            .collect();
        assert_eq!(mspn_eval(&bits, &steps), v,
            "failed for V = {}", v);
    }
}
```

### 8.2 Property-based test

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn mspn_complete(v in -15i64..=15) {
        let n = 4;
        let abs = v.abs() as u64;
        let bits: Vec<u8> = (0..n).rev()
            .map(|i| ((abs >> i) & 1) as u8)
            .collect();
        let steps: Vec<Micro> = bits.iter()
            .map(|&b| {
                if b == 0 { Micro::P }
                else if v >= 0 { Micro::F }
                else { Micro::R }
            })
            .collect();
        prop_assert_eq!(mspn_eval(&bits, &steps), v);
    }
}
```

### 8.3 Manual check for `n = 4`

| `V` | Bits | Steps |
|-----|------|-------|
| −15 | 1111 | rrrr  |
| −14 | 1110 | rrrp  |
| −13 | 1101 | rrpr  |
| ... | ...  | ...   |
| −1  | 0001 | pppr  |
| 0   | 0000 | pppp  |
| +1  | 0001 | pppf  |
| ... | ...  | ...   |
| +15 | 1111 | ffff  |

Every value has at least one encoding via its standard binary
representation. ✓

## 9. Open questions

The following are not addressed by the proof and remain open:

- **Canonical form** — Is there a unique encoding with minimal
  number of `r`/`f`?
- **Optimal encoding** — For a given V, which encoding minimizes
  the number of microsteps?
- **Extension to floating point** — How does MSPN extend to
  real numbers?
- **Hardware cost** — What is the gate-level cost of an MSPN
  accumulator vs. a two's complement adder?
- **Comparison to Booth encoding** — How does MSPN redundancy
  compare to Booth's signed-digit encoding used in multipliers?

## 10. Summary

MSPN is complete for all n ≥ 1 and all V in
[−(2ⁿ−1), +(2ⁿ−1)].

- MSPN is symmetric around zero.
- MSPN is redundant (3ⁿ encodings for 2^(n+1)−1 values).
- MSPN is mathematically equivalent to BSD (Avizienis, 1961),
  with dᵢ = s(mᵢ) × bᵢ.
- MSPN differs from BSD in encoding the sign in the microstep,
  not in the digit.
