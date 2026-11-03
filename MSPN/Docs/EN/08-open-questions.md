# Open Questions

## 1. Scope

This document lists all open questions about MSPN. They are
organized by category:

1. **Mathematical** — about the structure of MSPN itself
2. **Hardware** — about implementation cost and behavior
3. **Software** — about reference implementation and testing
4. **Applications** — about where MSPN is useful
5. **Formal verification** — about proving properties
6. **Terminology** — about naming and positioning

Each question is marked with a status:

- 🔴 **Blocking** — must be resolved before publication
- 🟡 **Important** — should be resolved before v1.0
- 🟢 **Open** — research question, no deadline

## 2. Mathematical questions

### Q1. What is the canonical form? 🟡

MSPN is redundant. For a given `V`, there are multiple encodings.
Which one is canonical?

**Candidates:**

- **Trivial** — standard binary with `f` for 1s (positive) or
  `r` for 1s (negative). Always length `n`.
- **NAF-based** — Non-Adjacent Form, minimizes non-zero digits.
  May be length `n+1`.
- **Shortest** — minimizes total number of positions.
- **Fewest μPC changes** — minimizes direction reversals.
- **Fewest total ticks** — counts `p` as 2 ticks.

**Problem:** These criteria may conflict. NAF may be shorter in
non-zero digits but longer in total length. No single canonical
form satisfies all criteria.

**See also:** `09-canonical-form.md` (if created).

### Q2. Does NAF map cleanly to MSPN? 🟡

NAF produces digits `{−1, 0, +1}`. Mapping:

```text
NAF +1 → MSPN (b=1, m=f)
NAF −1 → MSPN (b=1, m=r)
NAF  0 → MSPN (b=0, m=p)
```

**Problem 1:** NAF may require `n+1` positions. For `V = 2ⁿ−1`,
NAF is `1 0...0 −1` of length `n+1`. This exceeds the input width.

**Problem 2:** NAF minimizes non-zero digits, but `p` may still
cause μPC zigzag. Is a NAF-based encoding always "better" for
hardware?

**Open:** Is there an algorithm that maps `V` to an MSPN encoding
that is optimal for a given hardware metric?

### Q3. What is the distance between encodings of the same value? 🟢

For error correction, we need to know the Hamming distance
between different encodings of the same value. If the minimum
distance is d, then ⌊(d−1)/2⌋ errors can be corrected.

**Open:** What is the minimum Hamming distance between any two
MSPN encodings of the same `V`?

### Q4. How does MSPN extend to real numbers? 🟢

MSPN currently handles integers. Extending to floating point
requires:

- A fractional part (more positions after a binary point?)
- An exponent (how is it encoded?)
- A sign (already handled by microsteps)

**Open:** Is there a natural MSPN floating-point format? Does it
offer any advantage over IEEE 754?

### Q5. What is the relationship to non-adjacent forms with other bases? 🟢

NAF generalizes to base `b` with digits
`{−(b−1)/2, ..., +(b−1)/2}`. MSPN uses base 2 with ternary signs.
Is there a base-3 MSPN? Base-4? Does the theory generalize?

## 3. Hardware questions

### Q6. What is the hardware cost vs two's complement? 🔴

We need a concrete comparison:

- Number of LUTs
- Number of registers
- Critical path length
- Power consumption per operation

Current estimate (n=16): ~50 LUTs, ~25 registers.
But this is a rough estimate, not a synthesis result.

**Open:** What is the actual synthesized cost on a real FPGA
(e.g., iCE40, ECP5, Artix-7)?

### Q7. Can `p` be eliminated from the pipeline? 🟡

`p` requires a WAIT state (2 ticks instead of 1). If we
pre-compute the microstep sequence, we could skip p states
and save cycles.

**Trade-off:** Skipping requires lookahead. Regularity may be
more valuable than speed.

**Open:** Is the WAIT state worth eliminating?

### Q8. Can MSPN be parallelized? 🟢

Current MSPN is serial: one bit per tick. Can we process
multiple bits in parallel?

**Idea:** `n` accumulators, each processing one position, with
carry propagation between them. This is the classic parallel
prefix problem.

**Open:** What is the latency/area trade-off for parallel MSPN?

### Q9. How does MSPN map to native ternary hardware? 🟢

On a ternary FPGA (e.g., with memristors or multilevel cells),
MSPN could map directly: r, p, f become three voltage
levels.

**Open:** What is the design of a ternary MSPN cell? Is it
more efficient than binary MSPN?

### Q10. What is the power consumption per bit? 🟢

No data yet. Need to measure:

- Energy per tick
- Energy per bit
- Energy per operation (add, sub, shift)

**Open:** Is MSPN more or less energy-efficient than two's
complement for serial processing?

### Q11. Can the accumulator be made fully reversible? 🟡

Currently r undoes μPC but not ACC:

```text
f: μPC ← μPC + 1,  ACC ← (ACC << 1) + b
r: μPC ← μPC − 1,  ACC ← (ACC << 1) − b
```

Applying f then r restores μPC but not ACC. For full
reversibility (Bennett's principle), ACC must also be
restorable.

- Shift right on `r` instead of left (but then the bit order changes)
- Keep a log of operations (defeats the purpose)
- Use a different accumulator design

**Open:** Can MSPN be made fully reversible without a log?

## 4. Software questions

### Q12. Is the Rust reference implementation correct? 🔴

The current `src/lib.rs` is a simple implementation. It needs:

- Exhaustive tests for `n = 4, 8`
- Property-based tests with proptest
- Fuzzing with cargo-fuzz
- Formal verification (see Q15)

**Open:** Has the implementation been tested for all edge cases?

### Q13. What is the optimal encoding algorithm? 🟡

Given `V`, how do we compute the best MSPN encoding?

Candidates:

- Trivial: `O(n)` (just read binary)
- NAF-based: `O(n)` (using NAF algorithm)
- Optimal for ticks: `O(n²)` (dynamic programming)

**Open:** What is the fastest algorithm for each canonical form?

### Q14. Can MSPN be compiled from existing languages? 🟢

To use MSPN in practice, we need a compiler from C/Rust/Python
to MSPN. This requires:

- A target specification
- A code generator
- A runtime library

**Open:** Is it worth building an MSPN backend for LLVM or GCC?

## 5. Application questions

### Q15. Is MSPN better than 2-bit packed ternary for BitNet? 🔴

BitNet b1.58 uses 2-bit packed ternary weights. MSPN uses
binary activations + ternary signs. The comparison requires:

- Same accuracy
- Same speed (or faster)
- Same or less memory

**Open:** What is the benchmark result of MSPN vs bitnet.cpp?

### Q16. What would a reversible UART look like? 🟡

A reversible UART would:

- Read bits forward with `f`
- Detect errors with parity/CRC
- Re-read bits backward with `r` if error
- Correct and continue

**Open:** What is the protocol design? What is the error
correction rate?

### Q17. Is MSPN useful for education? 🟡

MSPN makes the data/sign separation visible. It could be a
teaching tool.

**Open:** What exercises, visualizations, and simulators would
be useful?

### Q18. Does MSPN help with neuromorphic computing? 🟢

Neuromorphic systems use spikes, not bits. MSPN is bit-based.
The mapping is not obvious.

**Open:** Is there a spike-based variant of MSPN? Would it
help?

## 6. Formal verification questions

### Q19. Can MSPN be formally verified? 🟡

The proof in 02-proof.md is informal. A formal proof in
Lean, Coq, or Isabelle would strengthen the result.

**Open:** What is the formal statement? How long is the proof?

### Q20. Can the Rust implementation be formally verified? 🟢

Tools like kani, creusot, or prusti can verify Rust code.

**Open:** Can `mspn_eval` be verified to match the formal
definition for all inputs?

## 7. Terminology questions

### Q21. MSPN, RFB, MSBN, or something else? 🔴

Three names are in use:

- MSPN — Microstep-Signed Positional Notation
- RFB — Reversible Forward-Backward encoding
- MSBN — Microstep-Signed Balanced Notation

Arguments:

- MSPN describes what it is (notation).
- RFB describes how it works (reversibility).
- MSBN describes the balanced variant.

Recommendation: Use MSPN as the primary name, RFB
as a property. But this needs to be decided before publication.

**Open:** What is the final name? Is there a conflict with
existing acronyms (MSPN = Mobile Station Packet Network)?

### Q22. How does MSPN relate to BSD? 🟡

MSPN is mathematically equivalent to BSD (Avizienis, 1961).
This is documented in 03-prior-art.md.

**Open:** Should the README lead with the BSD connection, or
with MSPN's specific contributions?

### Q23. Is "microstep" the right term? 🟢

"Microstep" is used in microprogramming (a single clock cycle
of a microinstruction). In MSPN, it is r, p, or f.

**Open:** Is there a better term? "Trit-action"? "Signed
bit-read"? "Tick"?

## 8. What is NOT an open question

To avoid confusion, here is what is already settled:

- ✅ Completeness — proved in `02-proof.md`
- ✅ Range — `[−(2ⁿ−1), +(2ⁿ−1)]`
- ✅ Symmetry — symmetric around zero
- ✅ Redundancy — `3ⁿ` encodings for `2^(n+1)−1` values
- ✅ Equivalent to BSD — with `dᵢ = s(mᵢ) × bᵢ`
- ✅ Canonical trivial encoding — standard binary with `f`/`r`
- ✅ Accumulator operation — `ACC ← (ACC << 1) + s × b`

## 9. Priority matrix

| Question | Status | Priority | Effort |
|---|---|---|---|
| Q1 Canonical form | 🟡 | High | Medium |
| Q2 NAF mapping | 🟡 | High | Medium |
| Q6 Hardware cost | 🔴 | Blocking | High |
| Q7 Eliminate `p` | 🟡 | Medium | Low |
| Q11 Full reversibility | 🟡 | High | High |
| Q12 Implementation correctness | 🔴 | Blocking | Low |
| Q15 BitNet benchmark | 🔴 | Blocking | High |
| Q21 Naming | 🔴 | Blocking | Low |
| Q19 Formal proof | 🟡 | Medium | High |

Blocking (🔴): Q6, Q12, Q15, Q21 — must be resolved
before publication.

Important (🟡): Q1, Q2, Q7, Q11, Q16, Q17, Q19, Q22 —
should be resolved before v1.0.

Open (🟢): Q3, Q4, Q5, Q8, Q9, Q10, Q13, Q14, Q18,
Q20, Q23 — research questions.

## 10. How to contribute

If you want to work on one of these questions:

- Open an issue on GitHub with the question number.
- Describe your approach.
- Submit a PR with:
  - A document in `docs/` (if theoretical)
  - Code in `src/` (if practical)
  - Tests (if verifiable)
- Update this file with the resolution.

All contributions are under AGPL-3.0-or-later.

## 11. Summary

MSPN has:

- 4 blocking questions — must be resolved before publication
- 8 important questions — should be resolved before v1.0
- 11 open questions — research directions

The most critical are:

- Q21 (Naming) — decide MSPN vs RFB vs MSBN
- Q6 (Hardware cost) — synthesize on FPGA
- Q12 (Implementation correctness) — exhaustive tests
- Q15 (BitNet benchmark) — measure against existing

Without these four, MSPN is a mathematical curiosity.
With them, it becomes a publishable, usable encoding.

For the formal definition, see `01-specification.md`.
For the proof, see `02-proof.md`.
For comparison, see `04-comparison.md`.
For applications, see `07-applications.md`.