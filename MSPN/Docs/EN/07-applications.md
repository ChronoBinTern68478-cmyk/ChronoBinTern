# Applications

## 1. Scope

This document describes where MSPN is useful, where it is not,
and where it is still speculative.

The applications are divided into three categories:

- **Category A** — fields that already use `{−1, 0, +1}`. MSPN
  offers an alternative encoding, not a new mathematical idea.
- **Category B** — fields where MSPN adds something specific:
  temporal encoding, reversibility, or native Boolean–ternary
  hybridization.
- **Category C** — speculative applications. Mathematically
  possible, but no evidence yet that they are practical.

**Important:** MSPN has **no killer application yet**. It is a
specialized tool looking for problems. This is not a failure —
many good ideas start this way. The three most promising
directions are identified in §5.

## 2. Category A — Fields that already use {−1, 0, +1}

These fields have working systems based on ternary or
signed-digit arithmetic. MSPN does not change the mathematics;
it changes how the digits are encoded and read.

### 2.1 Ternary machine learning — BitNet b1.58

In 2024, Microsoft Research introduced **BitNet b1.58**, a
Large Language Model (LLM) variant in which every parameter
(weight) is ternary `{−1, 0, +1}`, encoding approximately
**1.58 bits per weight** (`log₂ 3`)[reference:0].

Key properties of BitNet b1.58:

| Property | Value |
|---|---|
| Weight alphabet | `{−1, 0, +1}` |
| Bits per weight (information) | `log₂ 3 ≈ 1.585` |
| Bits per weight (storage) | 2 bits (typical), sub-2-bit formats exist |
| Matrix multiplication | Only integer **addition/subtraction** — no multiplication[reference:1] |
| Performance | Matches FP16/BF16 models of the same size[reference:2] |
| Open checkpoint | `microsoft/bitnet-b1.58-2B-4T`[reference:3] |

**Why this matters for MSPN:**

BitNet b1.58 uses **ternary weights** and **binary (or low-bit)
activations**. The matrix multiplication becomes:

```text
output = Σ (weight × activation)
       = Σ (±1 × b) or (0 × b)
       = Σ ±b
```

This is exactly the MSPN accumulator operation:

```text
ACC ← (ACC << 1) + s(m) × b
```

- weight = −1 → MSPN `r` (subtract)
- weight = +1 → MSPN `f` (add)
- weight = 0 → MSPN `p` (pause, no add)

MSPN is therefore a natural encoding for BitNet-style inference. The weight is the microstep; the activation is the bit. No multiplication is needed — only addition, subtraction, and shift.

Existing BitNet hardware:

- **bitnet.cpp** — CPU kernels for ternary LLM inference
- **LUT-based accelerators** — replace multiplication with conditional addition
- **t5b** — sub-2-bit storage using base-3 packing of five ternary values per byte at ~1.600 bits per weight

MSPN's contribution: It provides a formal, reversible, serial encoding for the weight–activation pair. Whether this is faster or smaller than existing encodings is an open question (see `08-open-questions.md`).

### 2.2 Booth multipliers

Booth encoding (1951) re-encodes sequences of 1s as +1 and
−1 at the boundaries. It uses digits {−1, 0, +1} and is
standard in hardware multipliers.

MSPN connection: Both use signed digits, but Booth is
optimized for reducing partial products in multiplication.
MSPN is optimized for serial, reversible reading. They
are complementary, not competing.

### 2.3 NAF in elliptic-curve cryptography

The Non-Adjacent Form (NAF) represents integers with digits
{−1, 0, +1} such that no two adjacent digits are non-zero.
It minimizes the number of non-zero digits (average density
1/3 versus 1/2 for binary) and is used in ECC scalar
multiplication.

MSPN connection: NAF can define a canonical MSPN form (see `09-canonical-form.md`). The temporal encoding of NAF could enable serial, reversible scalar multiplication, though this is speculative.

### 2.4 Balanced ternary CPUs

The Soviet Setun computer (1958) used balanced ternary
{−1, 0, +1} with weights 3ⁿ. It was efficient but required
ternary hardware.

MSPN connection: MSPN is not a balanced ternary
system. It uses binary weights (2ⁿ) with ternary signs.
It can emulate balanced ternary on binary hardware, but it
does not replace native ternary hardware.

## 3. Category B — Where MSPN adds something specific

These are fields where MSPN's temporal encoding, reversibility,
or Boolean–ternary hybrid offers a property that other methods
do not have.

### 3.1 Hybrid BNN/TWN with binary activations and ternary weights

Binary Neural Networks (BNN) use weights {−1, +1}.
Ternary Weight Networks (TWN) use {−1, 0, +1}.
Hybrid networks with binary activations and ternary weights
exist, but their encoding is awkward.

MSPN solution: A single MSPN stream encodes:

- the activation `b ∈ {0, 1}` (the bit)
- the weight `w ∈ {−1, 0, +1}` (the microstep)

No separate encoding is needed for weight and activation. The accumulator directly computes `Σ w × b`.

**Status:** Mathematically sound. Needs a benchmark against existing TWN/BNN encodings.

### 3.2 Bit-serial reversible protocols

Protocols like UART, SPI, and I2C read bits sequentially.
If a bit is corrupted, there is no way to go back without
re-transmitting the entire frame.

MSPN solution: `r` moves μPC backward and subtracts the bit. `f` moves forward and adds. A corrupted bit can be undone by `r`, re-read, and corrected with `f`.

**Status:** Useful for fault-tolerant serial links. But most protocols do not need reversibility. This is a niche.

### 3.3 Reversible computing

Reversible computing (Bennett, Fredkin) requires every
operation to have an exact inverse.

MSPN solution: `f` and `r` are exact inverses on μPC:

```text
f: μPC ← μPC + 1,  ACC ← (ACC << 1) + b
r: μPC ← μPC − 1,  ACC ← (ACC << 1) − b
```

Applying `f` then `r` (or `r` then `f`) restores μPC. The accumulator, however, is not restored because it shifts left each time. Full reversibility of the accumulator would require a different operation (e.g., shift right on `r`), which is an open design question.

**Status:** MSPN is partially reversible on μPC, not fully reversible on ACC. This is a limitation, not a feature.

### 3.4 Fault-tolerant encoding

MSPN has `3ⁿ` encodings for `2^(n+1) − 1` values. The excess can be used for error detection and correction.

**Status:** The redundancy is real, but MSPN is not competitive with dedicated ECC (Hamming, Reed–Solomon) for standard memory. It may be useful for serial links where redundancy is free (multiple microsteps per bit).

### 3.5 Education

MSPN makes the separation of data and sign explicit:

- The bit `b` is data.
- The microstep `m` is sign.
- The accumulator combines them.

In two's complement, this separation is hidden in the MSB. In MSPN, it is visible.

**Status:** MSPN is a good teaching tool for:

- number representations
- signed-digit arithmetic
- microprogramming
- reversible computing

This is a small but real contribution.

## 4. Category C — Speculative applications

These are mathematically possible but have no evidence of practical value yet.

| Application | Why speculative |
|---|---|
| Hardware security (fault injection) | Redundancy may help, but no evidence |
| Neuromorphic computing | MSPN is not spike-based; mismatch with SNN models |
| Quantum computing | Different paradigm; no clear mapping |
| DNA computing | `{−1, 0, +1}` is not natural for DNA |
| Optical computing | No evidence that ternary signs map to optical states |
| General-purpose CPU | Two's complement is better for this (see §6) |

**Honest assessment:** These should be mentioned in `08-open-questions.md`, not presented as applications.

## 5. The "killer app" question

MSPN has no killer application yet.

This is not a weakness. It is a research-stage reality. Many good ideas start as solutions looking for problems. The question is whether any of the three directions below will become a real use case.

### 5.1 Ternary ML (most promising)

BitNet b1.58 shows that ternary LLMs are practical. MSPN provides a natural encoding for ternary weights with binary activations. If MSPN can demonstrate a hardware or software advantage over existing 2-bit encodings, this becomes a real application.

**What is needed:** A benchmark comparing MSPN accumulator speed/size against 2-bit packed ternary and bitnet.cpp.

### 5.2 Reversible protocols (niche)

If reversible computing becomes practical, MSPN is ready. Until then, this is a niche for fault-tolerant serial links.

**What is needed:** A concrete protocol (e.g., a reversible UART) with measured error-correction performance.

### 5.3 Education (small but real)

MSPN is a good teaching tool today. It does not need a killer app to be useful in the classroom.

**What is needed:** A set of exercises and a visual simulator.

## 6. What MSPN is NOT for

MSPN is not a replacement for two's complement. It is a poor choice for:

| Application | Why not MSPN |
|---|---|
| General-purpose CPUs | Two's complement is universal, unique, and hardware-optimized |
| Operating systems | Every ABI assumes two's complement |
| Compilers and languages | Integer types are two's complement by standard |
| Memory-mapped I/O | Fixed-width words prefer unique encoding |
| Random-access data structures | Redundancy complicates comparison and hashing |
| Cryptographic primitives | Constant-time comparison needs unique representation |
| Databases | Indexing and sorting assume total order and unique keys |
| Networking stacks | Protocol headers assume fixed-width, unique fields |

In all these, two's complement's unique representation and ubiquitous hardware support outweigh any theoretical advantage of MSPN.

MSPN is a specialized encoding, not a general-purpose one.

## 7. Summary

| Category | Fields | Status |
|---|---|---|
| A | Ternary ML (BitNet b1.58), Booth, NAF/ECC, Setun | Existing systems; MSPN offers alternative encoding |
| B | Hybrid BNN/TWN, reversible protocols, reversible computing, fault tolerance, education | MSPN adds temporal/reversible properties; needs benchmarks |
| C | Security, neuromorphic, quantum, DNA, optical | Speculative; no evidence yet |

No killer app yet. Most promising: ternary ML and education.

Not for: general-purpose CPUs, OS, compilers, databases, cryptography (except ECC), networking.

For the formal definition, see `01-specification.md`.
For the proof, see `02-proof.md`.
For comparison, see `04-comparison.md`.
For open questions, see `08-open-questions.md`.