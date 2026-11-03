# Schematic

## 1. Overview

MSPN requires four hardware components:

1. **μPC** — microprogram counter (address of current microstep)
2. **Sign MUX** — converts `{r, p, f}` to `{−1, 0, +1}`
3. **Accumulator (ACC)** — shift register with add/subtract
4. **Control logic** — decodes microsteps, generates signals

The bit stream is read from a separate memory (ROM, RAM, or
serial input) at the address given by μPC.

## 2. Block diagram

```text
                ┌──────────────────┐
                │  Control Store   │
                │  (microprogram)  │
                └────────┬─────────┘
                         │ m ∈ {r,p,f}
                         ▼
                ┌──────────────────┐
                │       μPC        │◄──── reset / init
                │   (up/down)      │
                └────────┬─────────┘
                         │ address
                         ▼
                ┌──────────────────┐
                │  Bit Stream      │
                │  (ROM/RAM/serial)│
                └────────┬─────────┘
                         │ bit ∈ {0,1}
                         ▼
                ┌──────────────────┐
                │   Sign MUX       │
                │  r → −1          │
                │  p →  0          │
                │  f → +1          │
                └────────┬─────────┘
                         │ s ∈ {−1,0,+1}
                         │ bit ∈ {0,1}
                         ▼
                ┌──────────────────┐
                │  Multiplier      │  s × bit
                │  (trivial: AND)  │
                └────────┬─────────┘
                         │ ±bit
                         ▼
                ┌──────────────────┐
                │  Accumulator     │
                │  (shift + add)   │
                └────────┬─────────┘
                         │
                         ▼
                       result V
```

## 3. μPC — Microprogram Counter

### 3.1 Purpose

Holds the address of the current microstep. Drives both the
control store and the bit stream memory.

### 3.2 Behavior

| Microstep | μPC action |
|---|---|
| `r` | μPC ← μPC − 1 |
| `f` | μPC ← μPC + 1 |
| `p` | wait 1 tick, then μPC ← μPC + 1 |

### 3.3 Width

For an n-bit input, μPC must address n+1 positions
(NAF canonical form may need one extra position).
Minimum width: ceil(log₂(n+1)) bits.

| n | μPC width |
|---:|---:|
| 1 | 2 |
| 4 | 3 |
| 8 | 4 |
| 16 | 5 |
| 32 | 6 |

### 3.4 Implementation

```text
┌─────────────┐
│  Register   │
│  (k bits)   │
└──────┬──────┘
       │ Q
       ▼
┌─────────────┐     ┌──────────┐
│  Adder      │────►│  MUX     │
│  (+1/−1/0)  │     │          │
└─────────────┘     └────┬─────┘
       ▲                 │
       │                 ▼
┌──────┴──────┐    ┌─────────┐
│  Control    │    │  Clock  │
│  (r/f/p)    │    └─────────┘
└─────────────┘
```

**Signals:**

- `UP = 1` if `m = f`
- `DOWN = 1` if `m = r`
- `WAIT = 1` if `m = p`
- `LOAD = 1` if reset or external init

**Logic:**

```text
if LOAD:      μPC ← init
elif WAIT:    μPC ← μPC   (hold for 1 tick)
elif UP:      μPC ← μPC + 1
elif DOWN:    μPC ← μPC − 1
```

Note: p holds for exactly one tick, then on the next
tick μPC advances by +1. This requires a small state machine
or a WAIT_DONE flip-flop:

```text
WAIT_DONE_next = (m == p) && !WAIT_DONE
μPC_next =
    if WAIT_DONE: μPC + 1
    elif m == p:  μPC
    elif m == f:  μPC + 1
    elif m == r:  μPC − 1
```

## 4. Sign MUX

### 4.1 Purpose

Converts the 2-bit microstep code to a signed 2-bit value.

### 4.2 Encoding

| Microstep | 2-bit code | Sign value (2-bit) |
|---|---|---|
| `p` | `00` | `00 (0)` |
| `f` | `01` | `01 (+1)` |
| `r` | `10` | `11 (−1)` |
| (reserved) | `11` | — |

### 4.3 Implementation

A 2-to-2 lookup table, or directly:

```text
s[1] = m[1]                (sign bit)
s[0] = m[1] XOR m[0]       (magnitude bit, only for +1)
```

Or simplest: a 4-entry ROM.

## 5. Multiplier (s × bit)

### 5.1 Purpose

Multiply the signed 2-bit value by the 1-bit input.
Since the bit is 0 or 1, this is just a gate:

```text
±bit = s × b
```

### 5.2 Truth table

| `m` | `b` | `s` | `s × b` |
|---|---:|---:|---|
| `p` | 0 | 0 | 0 |
| `p` | 1 | 0 | 0 (invalid by constraint) |
| `f` | 0 | +1 | 0 (invalid by constraint) |
| `f` | 1 | +1 | +1 |
| `r` | 0 | −1 | 0 (invalid by constraint) |
| `r` | 1 | −1 | −1 |

Under the constraint b = 1 ⟺ m ∈ {r, f}, only 3 cases occur:

| `m` | `b` | `s × b` |
|---|---:|---:|
| `p` | 0 | 0 |
| `f` | 1 | +1 |
| `r` | 1 | −1 |

This is an AND gate plus sign extension.

## 6. Accumulator (ACC)

### 6.1 Purpose

Accumulates V = Σ s(mᵢ) × bᵢ × 2^(n−1−i) via shift-and-add.

### 6.2 Width

For an n-bit input, the range is [−(2ⁿ−1), +(2ⁿ−1)].
Required ACC width:

| n | Range | Min ACC width |
|---:|---|---:|
| 1 | −1..+1 | 2 |
| 4 | −15..+15 | 5 |
| 8 | −255..+255 | 9 |
| 16 | −65535..+65535 | 17 |
| 32 | ±(2³²−1) | 33 |

Recommendation: use n+1 signed bits minimum.
In software, use i32 or i64 for safety.

### 6.3 Behavior

```text
ACC ← (ACC << 1) + s × b
```

Per microstep:

| Microstep | Operation |
|---|---|
| `f` with `b=1` | ACC ← (ACC << 1) + 1 |
| `r` with `b=1` | ACC ← (ACC << 1) − 1 |
| `p` with `b=0` | ACC ← (ACC << 1) − 0 |

### 6.4 Implementation

```text
        ┌─────────────────────┐
        │   ACC Register      │
        │   (n+1 bits)        │
        └──────────┬──────────┘
                   │
                   ▼
        ┌─────────────────────┐
        │   Shift Left 1      │
        └──────────┬──────────┘
                   │
                   ▼
        ┌─────────────────────┐     ±bit
        │  Adder / Subtracter │◄────────
        └──────────┬──────────┘
                   │
                   ▼
        ┌─────────────────────┐
        │   MUX (load ACC)    │
        └──────────┬──────────┘
                   │
                   ▼
              (back to ACC)
```

**Signals:**

- `SHIFT = 1` (always, every tick)
- `ADD = 1` if `m = f` and `b = 1`
- `SUB = 1` if `m = r` and `b = 1`
- `ZERO = 1` if `m = p` (no add/sub, just shift)

The adder/subtracter can be implemented as:

```text
ACC_next = (ACC << 1) + (ADD ? +1 : SUB ? −1 : 0)
```

Since +1 and −1 are constants, this is a small incrementer/decrementer,
not a full adder.

### 6.5 Alternative: direct weighted sum

Instead of shift-and-add, you could use a weight register:

```text
W ← 2^(n−1)   (initial)
for each m:
    ACC ← ACC + s(m) × b × W
    W ← W >> 1
```

This is conceptually simpler but slower (one multiply per step).
Shift-and-add is preferred for hardware.

## 7. Control logic

### 7.1 Inputs

- m (2 bits from control store)
- b (1 bit from bit stream)
- clk
- reset

### 7.2 Outputs

- μPC_up, μPC_down, μPC_wait
- ACC_add, ACC_sub, ACC_shift
- WAIT_DONE (state)

### 7.3 State machine

```text
State: IDLE, RUN, WAIT

IDLE:
    if reset: μPC ← 0, ACC ← 0
    else: → RUN

RUN:
    fetch m from control store[μPC]
    fetch b from bit stream[μPC]
    if m == p:
        ACC ← (ACC << 1) − 0
        → WAIT
    elif m == f:
        ACC ← (ACC << 1) + b
        μPC ← μPC + 1
    elif m == r:
        ACC ← (ACC << 1) − b
        μPC ← μPC − 1

WAIT:
    μPC ← μPC + 1
    → RUN
```

## 8. Timing

### 8.1 Clock cycle per microstep

| Microstep | Cycles |
|---|---:|
| `f` | 1 |
| `r` | 1 |
| `p` | 2 (1 pause + 1 advance) |

### 8.2 Timing diagram (example r p f)

```text
clk:      1    2    3    4    5
          │    │    │    │    │
μPC:      3    2    2    3    4
          │    │    │    │    │
m:        r    p    p    f    —
          │    │    │    │    │
b:        1    0    0    1    —
          │    │    │    │    │
ACC:      0   −1   −2   −2   +2
          │    │    │    │    │
WAIT:     0    1    0    0    0
```

### 8.3 Throughput

For n-bit input with k non-zero bits:

- Minimum cycles: n + k_p (where k_p is number of p steps)
- Worst case: 2n (all p, each takes 2 cycles)

## 9. FPGA implementation notes

### 9.1 Resource estimate (n = 16)

| Component | LUTs | Registers |
|---|---:|---:|
| μPC (5 bits) | ~10 | 5 |
| Sign MUX | 2 | 0 |
| Multiplier (AND) | 1 | 0 |
| ACC (17 bits) | ~20 | 17 |
| Control FSM | ~15 | 3 |
| Total | ~50 | 25 |

Very small. Fits in any modern FPGA.

### 9.2 Critical path

The critical path is:

```text
μPC → Control Store → Sign MUX → ACC Adder → ACC Register
```

For an n-bit ACC, the adder is the bottleneck. For n = 16,
an incrementer/decrementer is a few LUT levels deep.

### 9.3 Suggested FPGA flow

- Write VHDL/Verilog or use Amaranth/Migen (Python HDL).
- Test with the Rust reference implementation.
- Synthesize for a small FPGA (e.g., iCE40, ECP5).
- Compare resource usage with two's complement adder.

## 10. Comparison with two's complement hardware

| Component | Two's complement | MSPN |
|---|---|---|
| Adder | n-bit full adder | (n+1)-bit incrementer/decrementer |
| Sign handling | implicit (MSB) | explicit (Sign MUX) |
| Shift | not needed | required (shift left each tick) |
| Control | simple | FSM with WAIT state |
| μPC | not needed (standard PC) | required (microprogram counter) |
| Throughput | 1 op/cycle | 1 bit/cycle (slower) |
| Latency | ~1 cycle | n cycles |

Conclusion: MSPN is slower than two's complement for
parallel arithmetic, but faster for sequential bit-serial
protocols where bits arrive one at a time.

## 11. Open hardware questions

- Pipelining — can multiple MSPN streams share one ACC?
- Parallel MSPN — can n accumulators process a word in
  parallel with carry propagation?
- Optimization — can the WAIT state for p be eliminated
  by pre-computing the sequence?
- Power — what is the energy per bit for MSPN vs two's
  complement?
- Ternary hardware — how would this map to a native
  ternary FPGA?

## 12. Summary

MSPN hardware consists of:

- μPC — small up/down counter with WAIT state
- Sign MUX — 4-entry lookup table
- Multiplier — single AND gate
- ACC — (n+1)-bit shift register with incrementer/decrementer
- Control FSM — 3-state machine (IDLE, RUN, WAIT)

Total resource cost is very small (~50 LUTs, ~25 registers for n=16).
The design is simple enough for an FPGA prototype and clear enough
for a formal specification.