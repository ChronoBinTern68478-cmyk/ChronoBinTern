# Prior Art: Binary Signed Digit (BSD)

## 1. Avizienis, 1961

Algirdas Avizienis introduced Binary Signed Digit (BSD) 
representation in "Signed-digit number representations for 
fast parallel arithmetic", IRE Transactions on Electronic 
Computers, 1961.

BSD uses digits from {−1, 0, +1} with weights 2ⁿ.

## 2. Mathematical equivalence

MSPN is mathematically equivalent to BSD. If we define:

    dᵢ = s(mᵢ) × bᵢ

where s(r) = −1, s(f) = +1, s(p) = 0, then dᵢ ∈ {−1, 0, +1}
and:

    V = Σ dᵢ × 2ⁱ

This is exactly BSD.

## 3. What MSPN adds

MSPN differs from BSD in three ways:

1. **Temporal encoding**: the sign is in the microstep, not in the digit
2. **Reversibility**: r and f are distinct operations on μPC
3. **Synchronization**: p serves as zero, pause, and step simultaneously

## 4. Independent rediscovery

The author independently arrived at BSD while developing 
ChronoBinTern, without prior knowledge of Avizienis's work.
This is a case of multiple discovery, not plagiarism.

## 5. Why MSPN is still useful

BSD is a number representation. MSPN is an engineering scheme 
for sequential, reversible processing of that representation.
BSD answers "what is the number?". MSPN answers "how do we 
read it, in time, in both directions, with synchronization?".

## 6. References

- Avizienis, A. (1961). Signed-digit number representations 
  for fast parallel arithmetic. IRE Transactions on Electronic 
  Computers, EC-10(3), 389–400.
- Parhami, B. (1990). Generalized signed-digit number systems.
- Koren, I. (2002). Computer Arithmetic Algorithms, 2nd ed.