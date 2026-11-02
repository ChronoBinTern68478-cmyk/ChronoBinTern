# 02-LUT: The 27-Row Table

## 1. Overview

The LUT (Look-Up Table) is a 27-row table in which each row represents a temporal code consisting of 7 microsteps. It is the codec's only "logic"—everything else (`B`, `T`, `Dec`, and context) is derived from it.

| Parameter | Value |
|-----------|---------:|
| Rows | 27 |
| Microsteps per row | 7 |
| Bits per microstep | 2 |
| Bits per row | 14 (data) + 2 (padding) = 16 |
| Total size | 27 × 16 = 432 bits = 54 bytes |

## 2. Row Format

```text
┌──────────────────────────────────────────────┐
│ Step 1 │ Step 2 │ … │ Step 7 │ Padding       │
│ 2 bits │ 2 bits │   │ 2 bits │ 2 bits (00)   │
└──────────────────────────────────────────────┘
= 14 bits of temporal code + 2 padding bits = 16 bits
```

The padding is `00` = `P`. This is safe: if the 16 bits are accidentally read as 8 steps, the eighth step is a pause and causes no harm.

### MSB↔LSB Ordering

To avoid ambiguity when packing into a `u16`, the layout is defined explicitly:

```text
microstep[0] → bits 15…14 (MSB of the 16-bit row)
microstep[1] → bits 13…12
...
microstep[6] → bits 3…2
padding      → bits 1…0 (always 00)
```

For calculating `B` and `T`, the order does not matter (`AND` is commutative). For the MPC and serialization, however, the order is critical—it determines the execution sequence and the position of each step in the 16-bit word.

## 3. Microstep Encoding

| Code | Microstep | Sign `s` | Binary (signed 2-bit) |
|---|---|---:|---:|
| `00` | `p` | 0 | 0 |
| `01` | `f` | +1 | +1 |
| `11` | `r` | −1 | −1 |
| `10` | reserved | — | −2 (unused) |

Why this encoding: the code is already the sign in two's complement. `r = 11₂ = −1` directly, with no decoding table. Two bits is the minimum (three states cannot fit in one bit), for maximum efficiency.

The reserved code `10` is treated as an error in the current prototype (`assert!`/`panic!` if encountered). Future versions could define it as `Q` (anti-`P`) for reversibility, but that is outside the scope of the canonical LUT.

## 4. Computing `B` and `T` from a Row

### Binary Part (`B`) — First 4 Steps

Weights: 8, 4, 2, 1 (positional binary).

$$
B = \sum_{i=0}^{3} s_i \cdot 2^{3-i}
$$

Mask-based optimization (from "Efficient Definition"):

```text
pos_mask = bits where the code is 01 (f)
neg_mask = bits where the code is 11 (r)

ACC = (v & pos_mask) - (v & neg_mask)
```

This is `O(1)` for any `N`—two AND operations and one subtraction, with no loop.

### Ternary Part (`T`) — Last 3 Steps

Weights: 9, 3, 1 (signed positional ternary).

$$
T = \sum_{i=0}^{2} s_i \cdot 3^{2-i}
$$

Note: masks do not work for `T` because the weights are not powers of 2. Use either a small LUT or directly extract the 3 codes:

```c
int8_t s0 = decode(t_code & 0x3);
int8_t s1 = decode((t_code >> 2) & 0x3);
int8_t s2 = decode((t_code >> 4) & 0x3);
int T = 9*s2 + 3*s1 + s0;
```

For fixed `N=3`, this is straightforward: three extractions and three additions.

### Worked Example (Step by Step)

Take `LUT[22]`, `Dec = +11`, microcode `F P F F | F P P`:

```text
Calculate B (first 4 steps, weights 8, 4, 2, 1):
  Step 1: F → +8
  Step 2: P →  0
  Step 3: F → +2
  Step 4: F → +1
  B = 8 + 0 + 2 + 1 = +11  ✓

Calculate T (last 3 steps, weights 9, 3, 1):
  Step 5: F → +9
  Step 6: P →  0
  Step 7: P →  0
  T = 9 + 0 + 0 = +9  ✓

Check the invariant:
  |T + 2| = |9 + 2| = 11 = |B|  ✓
```

This example shows how `B` and `T` are derived from the 7 microsteps and why the invariant `|T + 2| = |B|` holds.

## 5. LUT Indexing

The LUT can be indexed in two equivalent ways.

### By `Dec` (Signed)

$$
\text{index} = \text{Dec} + 11,\qquad \text{Dec} \in [-11, +15]
$$

| `Dec` | Index |
|---:|---:|
| −11 | 0 |
| 0 | 11 |
| +15 | 26 |

### By a Triple of Trits

$$
\text{index} = (a+1)\cdot 9 + (b+1)\cdot 3 + (c+1),
\qquad a,b,c \in \{-1,0,+1\}
$$

| Trits | Index |
|---|---:|
| (−1, −1, −1) | 0 |
| (0, 0, 0) | 13 |
| (+1, +1, +1) | 26 |

The two formulas are equivalent because:

$$
\text{Dec} + 11 = T + 13 = (a \cdot 9 + b \cdot 3 + c) + 13
$$

## 6. Canonical Offset

Our LUT satisfies:

$$
T = \text{Dec} - 2
$$

for all 27 rows. That is:

$$
\text{Dec} - T = 2
$$

always, regardless of the row. This is the only constant offset valid for both ranges:

* If `k < 2`: `T_max = 15 − k > 13` → exceeds the range.
* If `k > 2`: `T_min = −11 − k < −13` → falls below the range.
* If `k = 2`: both bounds match exactly.

**Claim:** Within the ranges `Dec ∈ [−11, +15]` and `T ∈ [−13, +13]`, exactly one LUT has a constant offset: `T = Dec − 2`. All other valid LUTs have a variable offset.

## 7. What the LUT Does NOT Contain

The LUT stores only the temporal code. The following columns from the documentation table are not stored:

| Column | In LUT memory? | Why |
|---|---|---|
| `Dec` (−11…+15) | ❌ No | Derived from `(B, T)` or from the MPC |
| Nibble (Bin) | ❌ No | This is `B`, computed from the first 4 steps |
| Balanced ternary | ❌ No | This is `T`, computed from the last 3 steps |
| Work cycles (F/R/P) | ✅ Yes | This is the only data that is stored |
| `Decb3` | ❌ No | This is `T`, derived |
| Hex | ❌ No | This is `B`, derived |

**Principle:** Everything else is derived. If the LUT stores only the code, there is no risk of a mismatch between "stored `B`" and "computed `B`"—they always agree because `B` is computed from the code.

## 8. Complete Table (27 Rows)

| Index | `Dec` | `B` | `T` | Microcode | Hex (`u16`) |
|---:|---:|---:|---:|---|---:|
| 0 | −11 | −11 | −13 | `R P R R R R R` | `0xCFFC` |
| 1 | −10 | −10 | −12 | `R P R P R R P` | `0xCCF0` |
| 2 | −9 | −9 | −11 | `R P P R R R F` | `0xC3F4` |
| 3 | −8 | −8 | −10 | `R P P P R P R` | `0xC0CC` |
| 4 | −7 | −7 | −9 | `P R R R R P P` | `0x3FC0` |
| 5 | −6 | −6 | −8 | `P R R P R P F` | `0x3CC4` |
| 6 | −5 | −5 | −7 | `P R P R R F R` | `0x33DC` |
| 7 | −4 | −4 | −6 | `P R P P R F P` | `0x30D0` |
| 8 | −3 | −3 | −5 | `P P R R R F F` | `0x0FD4` |
| 9 | −2 | −2 | −4 | `P P R P P R R` | `0x0C3C` |
| 10 | −1 | −1 | −3 | `P P P R P R P` | `0x0330` |
| 11 | 0 | 0 | −2 | `P P P P P R F` | `0x0034` |
| 12 | +1 | +1 | −1 | `P P P F P P R` | `0x010C` |
| 13 | +2 | +2 | 0 | `P P F P P P P` | `0x0400` |
| 14 | +3 | +3 | +1 | `P P F F P P F` | `0x0504` |
| 15 | +4 | +4 | +2 | `P F P P P F R` | `0x101C` |
| 16 | +5 | +5 | +3 | `P F P F P F P` | `0x1110` |
| 17 | +6 | +6 | +4 | `P F F P P F F` | `0x1414` |
| 18 | +7 | +7 | +5 | `P F F F F R R` | `0x157C` |
| 19 | +8 | +8 | +6 | `F P P P F R P` | `0x4070` |
| 20 | +9 | +9 | +7 | `F P P F F R F` | `0x4174` |
| 21 | +10 | +10 | +8 | `F P F P F P R` | `0x444C` |
| 22 | +11 | +11 | +9 | `F P F F F P P` | `0x4540` |
| 23 | +12 | +12 | +10 | `F F P P F P F` | `0x5044` |
| 24 | +13 | +13 | +11 | `F F P F F F R` | `0x515C` |
| 25 | +14 | +14 | +12 | `F F F P F F P` | `0x5450` |
| 26 | +15 | +15 | +13 | `F F F F F F F` | `0x5554` |

## 9. Performance

| Operation | Complexity | Cost |
|---|---:|---|
| Extract masks (setup) | `O(N)` | once at load time |
| Compute `B` | `O(1)` | 2 AND + 1 SUB |
| Compute `T` | `O(1)` | 3 decodes + 3 ADDs |
| Read a LUT row | `O(1)` | 1 memory access |
| Full cycle (7 steps) | `O(7)` | fixed |

## 10. Consistency Checks

### `compute_b_t` Function

```rust
fn compute_b_t(steps: &[MicroStep; 7]) -> (i8, i8) {
    let bin_weights: [i8; 4] = [8, 4, 2, 1];
    let tern_weights: [i8; 3] = [9, 3, 1];
    let mut b = 0i8;
    let mut t = 0i8;
    for i in 0..4 {
        b += match steps[i] {
            MicroStep::F => bin_weights[i],
            MicroStep::R => -bin_weights[i],
            MicroStep::P => 0,
        };
    }
    for i in 0..3 {
        t += match steps[4 + i] {
            MicroStep::F => tern_weights[i],
            MicroStep::R => -tern_weights[i],
            MicroStep::P => 0,
        };
    }
    (b, t)
}

```

### Validity Test

To ensure the LUT is valid, each of the 27 `(B, T)` pairs must be unique and satisfy:

$$
|T + 2| = |B|
$$

```rust
fn verify_lut() {
    let mut seen = std::collections::HashSet::new();
    for dec in -11..=15i8 {
        let (b, t) = compute_b_t(&LUT[(dec + 11) as usize]);
        assert!(seen.insert((b, t)), "Duplicate pair: ({}, {})", b, t);
        assert_eq!((t + 2).abs(), b.abs(), "Invariant violated at Dec={}", dec);
    }
}
```

This test catches any LUT error—including the three errors found in the first prototype (indices 2, 3, and 4).

## 11. The LUT as One of Many Valid Variants

### Space of Valid LUTs

Here, rows remain indexed by `Dec`, as in the complete table (`B = Dec`). A valid LUT must satisfy all of the following conditions:

- Each `(B, T)` pair occurs exactly once.
- `|T + 2| = |B|` for every pair.
- The 7 microsteps produce the correct `(B, T)`.

For `|B| = k`, where `1 ≤ k ≤ 11`, there are two valid values for `T`: `−2 − k` and `−2 + k`. Rows with `B = 0` and `|B| > 11` each have only one valid value for `T`.

### Number of Valid Mappings

Because `B` takes 27 distinct values (one per row), the `(B, T)` pairs are guaranteed to be unique regardless of the choice of `T`. Therefore, for each `k` from 1 to 11:

- The row `B = −k` can choose either `T = −2 − k` or `T = −2 + k` (2 choices).
- The row `B = +k` can independently choose either `T = −2 − k` or `T = −2 + k` (2 choices).

In total, this gives `4¹¹ = 2²² = 4,194,304` valid mappings, including the canonical one.

### Additional Requirement: `T` as a Permutation

If we additionally require each value of `T` to occur exactly once—that is, for the `T` column to be a permutation of `−13…+13`—then for each `k`:

- One of the rows `B = −k` and `B = +k` must use `T = −2 − k`.
- The other must use `T = −2 + k`.

This gives 2 choices for each `k` (which value is paired with the negative `B`), or `2¹¹ = 2048` valid mappings in total. The canonical LUT satisfies both requirements and is one of these 2048 variants.

### Why the Canonical LUT Is Special

Only the canonical LUT has the constant offset `T = Dec − 2` across all 27 rows. All other valid LUTs have a variable offset: `Dec − T` takes different values for different rows.

This makes the canonical LUT:

- **The easiest to verify** — one formula for every row.
- **The easiest to generate** — it can be computed programmatically instead of being stored.
- **A natural starting point** — all other variants can be viewed as modifications of it.

### A Concrete Alternative

Swap the `T` values for `B = −1` and `B = +1`:

```text
Canonical:
  Dec = −1, B = −1 → T = −3   (|T + 2| = 1 = |B|)
  Dec = +1, B = +1 → T = −1   (|T + 2| = 1 = |B|)

After the swap:
  Dec = −1, B = −1 → T = −1   (|T + 2| = 1 = |B|)
  Dec = +1, B = +1 → T = −3   (|T + 2| = 1 = |B|)
```

This is a valid alternative: the pairs remain unique, the invariant holds, the `T` column remains a permutation of `−13…+13`, and all other rows can remain unchanged. The offset is no longer constant.

### An Invalid Rotation Example

The simple rotation by 1, `T = Dec − 3` with wraparound in `T ∈ [−13, +13]`, is not a valid LUT:

```text
Dec = −11, B = −11 → T = +13: |T + 2| = 15 ≠ 11 = |B|
Dec = −10, B = −10 → T = −13: |T + 2| = 11 ≠ 10 = |B|
Dec =   0, B =   0 → T =  −3: |T + 2| =  1 ≠  0 = |B|
Dec = +15, B = +15 → T = +12: |T + 2| = 14 ≠ 15 = |B|
```

Therefore, this rotation is not an example of a valid alternative LUT. Arbitrary rotations or column permutations are not automatically valid—each mapping must be checked against the conditions above.

### Why Other Variants Exist

Different LUTs may have different properties:

- **Different number of steps** — if some rows use more F/R operations than others.
- **Different error resistance** — if some pairs are easier to verify.
- **Different obfuscation** — if the LUT is secret, different variants produce different "ciphers."
- **Different optimization** — if some instructions are executed more often.

For the prototype, the canonical variant is sufficient: it is the only one with a constant offset and is the easiest to test. Other variants are a subject for future research.

### Security Implications

If the LUT is secret (not publicly known), it acts as a decoding key. Different valid LUTs give different interpretations of the same data on disk. In other words:

- The same 4-bit nibbles on disk can mean completely different things depending on the LUT used.
- This is a form of obfuscation, not cryptographic security—it relies on keeping the LUT secret, not on computational complexity.

**Note:** The LUT described in this section is the canonical variant—the only one with the constant offset `T = Dec − 2`. With the index fixed by `Dec`, there are `2²² = 4,194,304` valid mappings under the three basic conditions, of which `2¹¹ = 2048` keep `T` as a permutation of `−13…+13` (as the canonical LUT does). The canonical variant is chosen for its simplicity, verifiability, and naturalness; other valid variants may be explored for optimization or obfuscation.

## 12. In-Memory Storage

The LUT is packed as `[u16; 27]` using MSB-first ordering:

```rust
// 27 rows × 16 bits = [u16; 27]
// Encoding: F=01, R=11, P=00, padding=00, MSB-first
const LUT_PACKED: [u16; 27] = [
    0xCFFC, 0xCCF0, 0xC3F4, 0xC0CC, 0x3FC0, 0x3CC4, 0x33DC,
    0x30D0, 0x0FD4, 0x0C3C, 0x0330, 0x0034, 0x010C, 0x0400,
    0x0504, 0x101C, 0x1110, 0x1414, 0x157C, 0x4070, 0x4174,
    0x444C, 0x4540, 0x5044, 0x515C, 0x5450, 0x5554,
];
```

Example decoding `LUT_PACKED[22] = 0x4540`:

```text
0x4540 = 0100 0101 0100 0000
        ││││ ││││ ││││ ││││
        ││││ ││││ ││││ └┴┴┴─ padding = 00
        ││││ ││││ └┴┴┴────── step 7: 00 = P
        ││││ └┴┴┴──────────── step 6: 00 = P
        ││││                    step 5: 01 = F
        ││││
        ││└┴──────────────────── step 4: 01 = F
        │└────────────────────── step 3: 01 = F
        └─────────────────────── steps 1-2: 01 00 = F P
```

Result: `F P F F | F P P` ✓ (matches row 22 in the table)

### Test for `LUT_PACKED`

```rust
fn verify_packed_lut() {
    for (idx, &packed) in LUT_PACKED.iter().enumerate() {
        // Decode the 7 steps from the 16-bit word
        let mut steps = [MicroStep::P; 7];
        for i in 0..7 {
            let code = (packed >> (14 - 2 * i)) & 0x3;
            steps[i] = match code {
                0b00 => MicroStep::P,
                0b01 => MicroStep::F,
                0b11 => MicroStep::R,
                _ => panic!("Reserved code 10 at index {}", idx),
            };
        }
        // Check that the padding is 00
        assert_eq!(packed & 0x3, 0, "Padding is not 00 at index {}", idx);
        // Check that B and T match the expected values
        let (b, t) = compute_b_t(&steps);
        let dec = idx as i8 - 11;
        assert_eq!(b, dec, "B mismatch at index {}", idx);
        assert_eq!(t, dec - 2, "T mismatch at index {}", idx);
    }
}
```

This test ensures that `LUT_PACKED` is consistent with the table and that the padding is always 00.

## 13. Summary

```text
LUT = 27 rows × 16 bits = 432 bits = 54 bytes
Each row = 7 microsteps × 2 bits + 2 padding bits
Microstep: p=00, f=01, r=11 (two's complement)
Indexing: Dec + 11 or (a+1)·9 + (b+1)·3 + (c+1)
Canonical offset: T = Dec − 2 (the only constant offset)
Invariant: |T + 2| = |B|
Ordering: MSB-first (microstep[0] → bits 15…14)
```

**Key property:** The LUT is a constant—it does not change with scale, does not depend on stream length, and contains nothing redundant. It is the core that remains fixed while everything else (accumulators, MPC, and disk) scales around it.