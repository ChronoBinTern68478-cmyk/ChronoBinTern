# Examples

## 1. Notation reminder

For `n` bits, positions `0..n−1` (MSB first):

- Position `i` has weight `Wᵢ = 2^(n−1−i)`.
- Bit `bᵢ ∈ {0, 1}`.
- Microstep `mᵢ ∈ {r, p, f}`.
- Constraint: `bᵢ = 1 ⟺ mᵢ ∈ {r, f}`, `bᵢ = 0 ⟺ mᵢ = p`.
- Sign: `s(r) = −1`, `s(f) = +1`, `s(p) = 0`.
- Value: `V = Σᵢ s(mᵢ) × bᵢ × 2^(n−1−i)`.

Accumulator update per position:

```text
ACC ← (ACC << 1) + s(m) × b

r with b=1: ACC ← (ACC << 1) − 1
f with b=1: ACC ← (ACC << 1) + 1
p with b=0: ACC ← (ACC << 1) − 0
```

**Note on ACC width:** For `n` bits, the accumulator must hold values in
`[−(2ⁿ−1), +(2ⁿ−1)]`. For `n=4`, a 5-bit signed register suffices; for
`n=8`, a 9-bit signed register; for `n=16`, a 17-bit signed register.
In software, use `i32` or `i64` to avoid overflow.

## 2. The canonical (trivial) encoding

The proof in [02-proof.md](./02-proof.md) constructs a simple encoding
for every value:

- If `V ≥ 0`, use `f` for 1-bits and `p` for 0-bits.
- If `V < 0`, use `r` for 1-bits and `p` for 0-bits.

This encoding is not the shortest possible (see §8), but it always
works and is easy to generate.

## 3. Complete table for `n = 4`

All 31 representable values in `[−15, +15]`, with their canonical
encoding:

| `V` | Bits | Steps | ACC trace |
|---:|---|---|---|
| −15 | `1111` | `r r r r` | `0, −1, −3, −7, −15` |
| −14 | `1110` | `r r r p` | `0, −1, −3, −7, −14` |
| −13 | `1101` | `r r p r` | `0, −1, −3, −6, −13` |
| −12 | `1100` | `r r p p` | `0, −1, −3, −6, −12` |
| −11 | `1011` | `r p r r` | `0, −1, −2, −5, −11` |
| −10 | `1010` | `r p r p` | `0, −1, −2, −5, −10` |
| −9 | `1001` | `r p p r` | `0, −1, −2, −4, −9` |
| −8 | `1000` | `r p p p` | `0, −1, −2, −4, −8` |
| −7 | `0111` | `p r r r` | `0, 0, −1, −3, −7` |
| −6 | `0110` | `p r r p` | `0, 0, −1, −3, −6` |
| −5 | `0101` | `p r p r` | `0, 0, −1, −2, −5` |
| −4 | `0100` | `p r p p` | `0, 0, −1, −2, −4` |
| −3 | `0011` | `p p r r` | `0, 0, 0, −1, −3` |
| −2 | `0010` | `p p r p` | `0, 0, 0, −1, −2` |
| −1 | `0001` | `p p p r` | `0, 0, 0, 0, −1` |
| 0 | `0000` | `p p p p` | `0, 0, 0, 0, 0` |
| +1 | `0001` | `p p p f` | `0, 0, 0, 0, +1` |
| +2 | `0010` | `p p f p` | `0, 0, 0, +1, +2` |
| +3 | `0011` | `p p f f` | `0, 0, 0, +1, +3` |
| +4 | `0100` | `p f p p` | `0, 0, +1, +2, +4` |
| +5 | `0101` | `p f p f` | `0, 0, +1, +2, +5` |
| +6 | `0110` | `p f f p` | `0, 0, +1, +3, +6` |
| +7 | `0111` | `p f f f` | `0, 0, +1, +3, +7` |
| +8 | `1000` | `f p p p` | `0, +1, +2, +4, +8` |
| +9 | `1001` | `f p p f` | `0, +1, +2, +4, +9` |
| +10 | `1010` | `f p f p` | `0, +1, +2, +5, +10` |
| +11 | `1011` | `f p f f` | `0, +1, +2, +5, +11` |
| +12 | `1100` | `f f p p` | `0, +1, +3, +6, +12` |
| +13 | `1101` | `f f p f` | `0, +1, +3, +6, +13` |
| +14 | `1110` | `f f f p` | `0, +1, +3, +7, +14` |
| +15 | `1111` | `f f f f` | `0, +1, +3, +7, +15` |

## 4. Redundancy: same bits, different microsteps

Because each 1-bit can be read with either `f` (add) or `r` (subtract),
the same bit pattern can yield different values.

### 4.1 Pattern `0011` (`n = 4`)

| Steps | Interpretation | `V` |
|---|---|---:|
| `p p f f` | `+2 + 1` | +3 |
| `p p f r` | `+2 − 1` | +1 |
| `p p r f` | `−2 + 1` | −1 |
| `p p r r` | `−2 − 1` | −3 |

The same bit pattern `0011` produces four distinct values.

### 4.2 Pattern `0001` (`n = 4`)

| Steps | Interpretation | `V` |
|---|---|---:|
| `p p p f` | `+1` | +1 |
| `p p p r` | `−1` | −1 |

### 4.3 Pattern `0000` (`n = 4`)

Only `p p p p` is valid (there are no 1-bits), yielding `V = 0`.

## 5. Redundancy: same value, different bits

Because of the constraint `bᵢ = 1 ⟺ mᵢ ∈ {r, f}`, different bit
patterns can yield the same value.

### 5.1 `V = +1` (`n = 4`)

| Bits | Steps | Interpretation | `V` |
|---|---|---|---:|
| `0001` | `p p p f` | `+1` | +1 |
| `0011` | `p p f r` | `+2 − 1` | +1 |

### 5.2 `V = +2` (`n = 4`)

| Bits | Steps | Interpretation | `V` |
|---|---|---|---:|
| `0010` | `p p f p` | `+2` | +2 |
| `0110` | `p f r p` | `+4 − 2` | +2 |

### 5.3 `V = −1` (`n = 4`)

| Bits | Steps | Interpretation | `V` |
|---|---|---|---:|
| `0001` | `p p p r` | `−1` | −1 |
| `0011` | `p p r f` | `−2 + 1` | −1 |

Redundancy is a feature, not a bug: it allows canonical forms (see §8)
and hardware optimization.

## 6. Detailed accumulator trace

### 6.1 `V = −11`, `n = 4`, steps `r p r r`

| Step | `m` | `b` | Formula | ACC (binary, 8-bit) | ACC (decimal) |
|---:|---|---:|---|---|---:|
| 0 | — | — | init | `00000000` | 0 |
| 1 | `r` | 1 | `(0 << 1) − 1` | `11111111` | −1 |
| 2 | `p` | 0 | `(−1 << 1) − 0` | `11111110` | −2 |
| 3 | `r` | 1 | `(−2 << 1) − 1` | `11111011` | −5 |
| 4 | `r` | 1 | `(−5 << 1) − 1` | `11110101` | −11 |

Final: `ACC = 11110101₂ = −11₁₀`.

### 6.2 `V = +11`, `n = 4`, steps `f p f f`

| Step | `m` | `b` | Formula | ACC (binary, 8-bit) | ACC (decimal) |
|---:|---|---:|---|---|---:|
| 0 | — | — | init | `00000000` | 0 |
| 1 | `f` | 1 | `(0 << 1) + 1` | `00000001` | +1 |
| 2 | `p` | 0 | `(+1 << 1) + 0` | `00000010` | +2 |
| 3 | `f` | 1 | `(+2 << 1) + 1` | `00000101` | +5 |
| 4 | `f` | 1 | `(+5 << 1) + 1` | `00001011` | +11 |

Final: `ACC = 00001011₂ = +11₁₀`.

### 6.3 `V = 0`, `n = 4`, steps `p p p p`

| Step | `m` | `b` | Formula | ACC |
|---:|---|---:|---|---:|
| 0 | — | — | init | 0 |
| 1 | `p` | 0 | `(0 << 1) + 0` | 0 |
| 2 | `p` | 0 | `(0 << 1) + 0` | 0 |
| 3 | `p` | 0 | `(0 << 1) + 0` | 0 |
| 4 | `p` | 0 | `(0 << 1) + 0` | 0 |

Final: `ACC = 0`. There is no `+0` / `−0` distinction. ✓

## 7. Large example (8 bits)

### 7.1 Bit pattern `10000001`

Positions and weights:

| `i` | `bᵢ` | Weight |
|---:|---:|---:|
| 0 | 1 | 128 |
| 1 | 0 | 64 |
| 2 | 0 | 32 |
| 3 | 0 | 16 |
| 4 | 0 | 8 |
| 5 | 0 | 4 |
| 6 | 0 | 2 |
| 7 | 1 | 1 |

### 7.2 `V = +129`, steps `f p p p p p p f`

| Step | `m` | `b` | ACC |
|---:|---|---:|---:|
| 0 | — | — | 0 |
| 1 | `f` | 1 | 1 |
| 2 | `p` | 0 | 2 |
| 3 | `p` | 0 | 4 |
| 4 | `p` | 0 | 8 |
| 5 | `p` | 0 | 16 |
| 6 | `p` | 0 | 32 |
| 7 | `p` | 0 | 64 |
| 8 | `f` | 1 | 129 |

Final: `ACC = +129₁₀ = 10000001₂`.

### 7.3 `V = −129`, steps `r p p p p p p r`

| Step | `m` | `b` | ACC |
|---:|---|---:|---:|
| 0 | — | — | 0 |
| 1 | `r` | 1 | −1 |
| 2 | `p` | 0 | −2 |
| 3 | `p` | 0 | −4 |
| 4 | `p` | 0 | −8 |
| 5 | `p` | 0 | −16 |
| 6 | `p` | 0 | −32 |
| 7 | `p` | 0 | −64 |
| 8 | `r` | 1 | −129 |

Final: `ACC = −129₁₀`. This is symmetric to `+129`. ✓

## 8. NAF-based canonical form

The canonical encoding from §2 does not minimize the number of
non-zero digits. NAF (Non-Adjacent Form) minimizes non-zero digits and
can be mapped to MSPN:

```text
NAF +1 → MSPN: b=1, m=f
NAF −1 → MSPN: b=1, m=r
NAF  0 → MSPN: b=0, m=p
```

### 8.1 `V = +3`, `n = 4`

| Form | Steps | Non-zero digits | Length |
|---|---|---:|---:|
| Trivial | `p p f f` | 2 | 4 |
| NAF | `p f p r` | 2 | 4 |

Both have 2 non-zero digits. NAF is not shorter here.

`NAF(3) = 1 0 −1`. For `n=4`, it is padded to `0 1 0 −1`.
The MSPN encoding is `p f p r`, interpreted as `+4 − 1 = +3`. ✓

### 8.2 `V = +7`, `n = 4`

| Form | Steps | Non-zero digits | Length |
|---|---|---:|---:|
| Trivial | `p f f f` | 3 | 4 |
| NAF | `f p p r` | 2 | 4 |

NAF saves one non-zero digit: `1 0 0 −1 = 8 − 1 = 7`. ✓

### 8.3 `V = +15`, `n = 4` (length issue)

| Form | Steps | Non-zero digits | Length |
|---|---|---:|---:|
| Trivial | `f f f f` | 4 | 4 |
| NAF | `f p p p r` | 2 | 5 |

`NAF(15) = 1 0 0 0 −1` requires 5 positions, not 4. For `n=4`,
this exceeds the input width.

**Consequence:** NAF minimizes the number of non-zero digits but may
increase the length. This is the trade-off noted in
`08-open-questions.md`.

## 9. Edge cases

### 9.1 `V = 0`

Unique encoding: `p p p p ... p`. There is no `+0` / `−0` distinction.

### 9.2 `V = +(2ⁿ − 1)`

All bits are 1 and all steps are `f`. For `n=4`: `f f f f = +15`.

### 9.3 `V = −(2ⁿ − 1)`

All bits are 1 and all steps are `r`. For `n=4`: `r r r r = −15`.

### 9.4 `V = +1`

Canonical: `0001 pppf`. Alternative: `0011 ppfr`.

### 9.5 `V = −1`

Canonical: `0001 pppr`. Alternative: `0011 pprf`.

### 9.6 Single position, `n = 1`

- `0` with `p` → 0
- `1` with `f` → +1
- `1` with `r` → −1

Full coverage, no redundancy (`3¹ = 3` encodings, 3 values). ✓

## 10. Summary

| Property | Observation |
|---|---|
| Completeness | Every `V` in `[−(2ⁿ−1), +(2ⁿ−1)]` has an encoding |
| Symmetry | `+V` and `−V` have mirror encodings |
| Redundancy | Same bits can yield different values; same value can have different bits |
| Zero | Unique encoding (all `p`), no `+0` / `−0` |
| NAF-canonical | Minimizes non-zero digits, may increase length |
| ACC width | Needs at least `n+1` signed bits for an `n`-bit input |
