# Comparison with Other Representations

## 1. Scope

This document compares MSPN with other methods for representing
signed integers in binary form.

Seven methods are mentioned. Five are compared in detail.

- **Compared in detail:** two's complement, sign-magnitude,
  BSD (Avizienis, 1961), balanced ternary, MSPN.
- **Mentioned for context:** one's complement, offset binary,
  Booth encoding / NAF.

The comparison focuses on four criteria that matter for MSPN's
intended applications:

1. **Symmetry around zero**
2. **Negative values without an extra sign bit**
3. **Reversibility**
4. **Redundancy**

## 2. The seven methods

### 2.1 Two's complement
Standard for modern CPUs. Negative numbers are obtained by
bitwise inversion plus 1. Range for `n` bits: `−2^(n−1) .. 2^(n−1)−1`.
Unique representation. Asymmetric.

### 2.2 Sign-magnitude
Most significant bit is the sign, remaining bits are the magnitude.
Range: `−2^(n−1)+1 .. 2^(n−1)−1`. Two zeros (`+0` and `−0`).

### 2.3 One's complement
Negative numbers are obtained by bitwise inversion. Range:
`−2^(n−1)+1 .. 2^(n−1)−1`. Two zeros. Requires end-around carry.

### 2.4 Offset binary (excess-K)
Value is stored as unsigned with a bias. Range for offset `2^(n−1)`:
`−2^(n−1) .. 2^(n−1)−1`. Unique. Used in A/D converters and
IEEE 754 exponents.

### 2.5 BSD (Avizienis, 1961)
Binary Signed Digit. Each digit is from `{−1, 0, +1}` with weights
`2ⁿ`. Range: `−(2ⁿ−1) .. +(2ⁿ−1)`. Redundant (`3ⁿ` encodings for
`2^(n+1)−1` values). Designed for carry-free addition.

### 2.6 Balanced ternary
Each digit is from `{−1, 0, +1}` with weights `3ⁿ`. Range for `n`
digits: `−(3ⁿ−1)/2 .. +(3ⁿ−1)/2`. Unique (canonical form exists).
Symmetric. Requires ternary hardware or ternary emulation.

### 2.7 Booth encoding / NAF
Signed-digit representations used in multiplication and
exponentiation. Booth encoding reduces partial products.
NAF (Non-Adjacent Form) minimizes the number of non-zero digits
(average density 1/3). Both use `{−1, 0, +1}`.

## 3. Formal comparison (five methods)

### 3.1 Range and encodings

| Method | Range (n bits) | Values | Encodings | Zeros |
|---|---|---:|---:|---:|
| Two's complement | `−2^(n−1) .. 2^(n−1)−1` | `2ⁿ` | `2ⁿ` (unique) | 1 |
| Sign-magnitude | `−(2^(n−1)−1) .. 2^(n−1)−1` | `2ⁿ−1` | `2ⁿ` | 2 |
| BSD | `−(2ⁿ−1) .. +(2ⁿ−1)` | `2^(n+1)−1` | `3ⁿ` (redundant) | 1 |
| Balanced ternary | `−(3ⁿ−1)/2 .. +(3ⁿ−1)/2` | `3ⁿ` | `3ⁿ` (unique) | 1 |
| **MSPN** | `−(2ⁿ−1) .. +(2ⁿ−1)` | `2^(n+1)−1` | `3ⁿ` (redundant) | 1 |

### 3.2 Symmetry around zero

| Method | Symmetric? |
|---|---|
| Two's complement | ❌ No (`−2^(n−1)` vs `+2^(n−1)−1`) |
| Sign-magnitude | ✅ Yes |
| BSD | ✅ Yes |
| Balanced ternary | ✅ Yes |
| **MSPN** | ✅ Yes |

### 3.3 Negative values without an extra sign bit

| Method | Needs sign bit? | How sign is encoded |
|---|---|---|
| Two's complement | ❌ No | MSB has negative weight |
| Sign-magnitude | ✅ Yes | Dedicated sign bit |
| BSD | ❌ No | Digit is `{−1, 0, +1}` |
| Balanced ternary | ❌ No | Digit is `{−1, 0, +1}` |
| **MSPN** | ❌ No | Microstep `{r, p, f}` |

### 3.4 Reversibility

| Method | Reversible reading? |
|---|---|
| Two's complement | ❌ No |
| Sign-magnitude | ❌ No |
| BSD | ❌ No (notation only) |
| Balanced ternary | ❌ No (notation only) |
| **MSPN** | ✅ Yes (`r` and `f` are distinct) |

### 3.5 Redundancy

| Method | Redundant? | Encodings per value |
|---|---|---|
| Two's complement | ❌ No | 1 |
| Sign-magnitude | ❌ No | 1 |
| BSD | ✅ Yes | Multiple |
| Balanced ternary | ❌ No (canonical) | 1 |
| **MSPN** | ✅ Yes | Multiple |

## 4. Detailed analysis per criterion

### 4.1 Symmetry around zero

Only sign-magnitude, BSD, balanced ternary and MSPN are symmetric.
Two's complement is asymmetric because the MSB carries both value
and sign, which forces `−2^(n−1)` to have no positive counterpart.

MSPN's symmetry is a direct consequence of BSD's symmetry: since
each digit is from `{−1, 0, +1}`, every positive value has an
exact negative counterpart.

**Consequence:** MSPN never suffers from the `abs(INT_MIN) < 0`
problem that plagues two's complement.

### 4.2 Negative values without an extra sign bit

Two's complement and MSPN both avoid a dedicated sign bit, but
for different reasons:

- **Two's complement:** the MSB itself has negative weight. The
  sign is *implicit in the position*.
- **MSPN:** the sign is carried by the microstep. The sign is
  *explicit in the reading action*.

Sign-magnitude needs a dedicated bit. BSD and balanced ternary
encode the sign in the digit itself.

**Consequence:** MSPN preserves the full `n`-bit range for
magnitude, like two's complement, but without the asymmetry.

### 4.3 Reversibility

Only MSPN is reversible among the five compared methods.

In BSD and balanced ternary, `+1` and `−1` are stored in the digit.
There is no "direction of reading". The representation is static.

In MSPN, `r` and `f` are *operations on μPC*, not just signs.
Reading the same bit sequence with `r` versus `f` gives different
values:

```text
1011 with r p r r  →  −11
1011 with f p f f  →  +11
```

This is not possible in any other method listed here.

**Consequence:** MSPN is suitable for bidirectional protocols,
reversible computing, and undo/redo operations.

### 4.4 Redundancy

BSD and MSPN are redundant. Two's complement, sign-magnitude, and
balanced ternary are unique.

Redundancy has two sides:

- **Advantage:** Multiple encodings allow optimization (fewer
  non-zero digits, shorter sequences, canonical forms via NAF).
- **Disadvantage:** Values cannot be compared by simple bit
  comparison; normalization is required.

MSPN inherits BSD's redundancy but adds a new dimension: the same
value can be encoded with different microstep sequences, not
just different digits.

**Consequence:** MSPN is more flexible than BSD in encoding, but
also more complex to normalize.

## 5. Where MSPN fits

MSPN is a good fit for:

| Application | Why MSPN |
|---|---|
| Reversible computing | `r` and `f` are exact inverses on μPC |
| Bidirectional serial protocols | `r` = backward read, `f` = forward read |
| Hybrid Boolean × ternary systems | Bits are Boolean, signs are ternary |
| Ternary-weight neural networks | `r` → −1, `f` → +1, `p` → 0 |
| Educational demonstrations | Explicit separation of data and sign |
| Bit-serial arithmetic | Natural mapping to sequential streams |

In all these applications, MSPN offers properties that two's
complement and sign-magnitude do not have: reversibility, built-in
synchronization, and native ternary sign encoding.

## 6. Where MSPN does NOT fit

MSPN is not a replacement for two's complement. It is a poor
choice for:

| Application | Why not MSPN |
|---|---|
| General-purpose CPUs | Two's complement is universal, unique, and hardware-optimized |
| Operating systems | Every ABI assumes two's complement |
| Compilers and languages | Integer types are two's complement by standard |
| Memory-mapped I/O | Fixed-width words prefer unique encoding |
| Random-access data structures | Redundancy complicates comparison and hashing |
| Cryptographic primitives | Constant-time comparison needs a unique representation |

In all these applications, two's complement's unique representation
and hardware support outweigh MSPN's theoretical advantages.

## 7. Honest conclusion

MSPN is not a universal replacement for any standard method.

It is a specialized encoding for applications where:

- Symmetry around zero matters (no `abs(INT_MIN) < 0`).
- Negative values must be representable without an extra sign bit.
- Reversibility is required (bidirectional reading).
- Redundancy is acceptable or desirable (multiple encodings,
  canonical forms via NAF).

For general-purpose computing, two's complement remains the best
choice: it is unique, hardware-efficient, and universally supported.

For carry-free arithmetic, BSD remains the best choice: it was
designed specifically for that purpose.

For ternary hardware, balanced ternary remains the best choice: it
uses the hardware efficiently.

MSPN's niche is the intersection of:

- Hybrid Boolean–ternary systems
- Reversible computing
- Sequential bit-serial protocols
- Ternary-weight neural networks

In these domains, MSPN offers a combination of properties that
no other method provides: symmetric range without a sign bit,
reversible reading, built-in synchronization, and native support
for ternary signs.

The choice between MSPN and two's complement is therefore not
about superiority, but about fit to the application.

## 8. Related signed-digit methods

Two additional methods are worth mentioning, though not compared
in detail:

### 8.1 Booth encoding (1951)

Reduces partial products in multiplication by re-encoding
sequences of 1s as +1 and −1 at the boundaries. Uses digits
`{−1, 0, +1}`. Not unique. Optimized for hardware multipliers.

**Difference from MSPN:** Booth is for multiplication, while MSPN
is for representation and reversible reading.

### 8.2 Non-Adjacent Form (NAF)

A signed-digit representation with no two adjacent non-zero
digits. Unique for each value. Average non-zero density is 1/3,
versus 1/2 for standard binary. Used in elliptic-curve
cryptography.

**Connection to MSPN:** NAF can be used to define a canonical
MSPN form that minimizes the number of `r`/`f`. This addresses
Open Question #1 in [02-proof.md](./02-proof.md). See
`09-canonical-form.md`.

## 9. Summary table (all seven methods)

| Method | Digits | Range (n) | Unique? | Reversible? | Ternary sign? |
|---|---|---|---|---|---|
| Two's complement | `{0,1}` | `−2^(n−1)..2^(n−1)−1` | ✅ | ❌ | ❌ |
| Sign-magnitude | `{0,1}` + sign | `−(2^(n−1)−1)..2^(n−1)−1` | ✅ | ❌ | ❌ |
| One's complement | `{0,1}` | `−(2^(n−1)−1)..2^(n−1)−1` | ✅ | ❌ | ❌ |
| Offset binary | `{0,1}` | `−2^(n−1)..2^(n−1)−1` | ✅ | ❌ | ❌ |
| BSD | `{−1,0,+1}` | `−(2ⁿ−1)..2ⁿ−1` | ❌ | ❌ | ✅ |
| Balanced ternary | `{−1,0,+1}` | `−(3ⁿ−1)/2..(3ⁿ−1)/2` | ✅ | ❌ | ✅ |
| MSPN | `{−1,0,+1}` via `{r,p,f}` | `−(2ⁿ−1)..2ⁿ−1` | ❌ | ✅ | ✅ |
| Booth / NAF | `{−1,0,+1}` | varies | NAF: ✅ | ❌ | ✅ |

## 10. References

- Avizienis, A. (1961). Signed-digit number representations for
  fast parallel arithmetic. *IRE Transactions on Electronic
  Computers, EC-10*(3), 389–400.
- Booth, A. D. (1951). A signed binary multiplication technique.
  *Quarterly Journal of Mechanics and Applied Mathematics, 4*(2),
  236–240.
- Parhami, B. (1990). Generalized signed-digit number systems: A
  unifying framework for redundant number representations.
  *IEEE Transactions on Computers, 39*(1), 89–98.
- Solinas, J. A. (2000). Efficient arithmetic on Koblitz curves.
  *Designs, Codes and Cryptography, 19*(2–3), 195–249. (NAF in
  elliptic-curve cryptography.)
- Koren, I. (2002). *Computer Arithmetic Algorithms* (2nd ed.).
  A K Peters.