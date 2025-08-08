# Error Correction Coding Theory

This document provides the mathematical foundations and theoretical background for the error correction coding algorithms implemented in CodeCrypt.

## Table of Contents

- [Introduction](#introduction)
- [Channel Coding Fundamentals](#channel-coding-fundamentals)
- [Linear Block Codes](#linear-block-codes)
- [Convolutional Codes](#convolutional-codes)
- [Maximum Likelihood Decoding](#maximum-likelihood-decoding)
- [Distance Properties](#distance-properties)
- [Performance Analysis](#performance-analysis)
- [Channel Models](#channel-models)

## Introduction

Error correction coding is a fundamental technique in digital communications and data storage systems. It involves adding structured redundancy to information to enable detection and correction of errors introduced during transmission or storage.

### Basic Concepts

**Information Theory Foundation:**
- **Shannon's Channel Coding Theorem**: For any channel with capacity C, there exist codes that can achieve arbitrarily low error probability for rates R < C.
- **Channel Capacity**: C = max I(X;Y) where I(X;Y) is mutual information between input X and output Y.

**Key Terminology:**
- **Codeword**: A sequence of symbols from a code
- **Code Rate**: R = k/n where k is information bits and n is total bits
- **Hamming Distance**: Number of positions where two sequences differ
- **Minimum Distance**: d_min = minimum Hamming distance between any two codewords

### Error Correction vs. Detection

| Capability | Requirement | Example |
|------------|-------------|---------|
| Detect t errors | d_min ≥ t + 1 | Parity check (d_min = 2, detect 1 error) |
| Correct t errors | d_min ≥ 2t + 1 | Hamming code (d_min = 3, correct 1 error) |
| Correct t, detect s errors | d_min ≥ t + s + 1 | Extended codes |

## Channel Coding Fundamentals

### System Model

```
Information → [Encoder] → Channel → [Decoder] → Information
    Bits         Codeword    Noise     Received      Estimate
```

**Encoding Process:**
1. Map k information bits to n-bit codeword
2. Add redundancy according to code structure
3. Transmit codeword through noisy channel

**Decoding Process:**
1. Receive potentially corrupted sequence
2. Apply decoding algorithm to estimate transmitted codeword
3. Extract information bits from estimated codeword

### Code Parameters

**Block Code Notation: (n, k, d)**
- n: Codeword length (total bits)
- k: Information length (message bits)
- d: Minimum Hamming distance

**Rate and Redundancy:**
- Code Rate: R = k/n
- Redundancy: r = n - k bits
- Efficiency: Higher rate → less redundancy → lower error correction capability

### Bounds on Code Parameters

**Hamming Bound (Sphere-Packing):**
For a binary code correcting t errors:
```
2^k ≤ 2^n / Σ(i=0 to t) C(n,i)
```

**Singleton Bound:**
```
d ≤ n - k + 1
```

**Gilbert-Varshamov Bound:**
There exists a code with parameters (n, k, d) if:
```
2^(n-k) > Σ(i=0 to d-2) C(n-1,i)
```

## Linear Block Codes

### Mathematical Structure

A linear block code C is a k-dimensional subspace of the vector space F₂ⁿ.

**Generator Matrix G:**
- Dimensions: k × n
- Each codeword: c = mG where m is information vector
- Systematic form: G = [I_k | P] where I_k is k×k identity matrix

**Parity Check Matrix H:**
- Dimensions: (n-k) × n
- Satisfies: GH^T = 0
- For systematic codes: H = [P^T | I_(n-k)]

### Syndrome Decoding

**Syndrome Calculation:**
```
s = rH^T
```
where r is the received vector.

**Properties:**
- s = 0 if no errors detected
- s = eH^T where e is error pattern
- Syndrome depends only on error pattern, not transmitted codeword

**Decoding Algorithm:**
1. Calculate syndrome s = rH^T
2. Find error pattern e such that eH^T = s
3. Correct: ĉ = r - e
4. Extract information: m̂ = first k bits of ĉ

### Standard Array Decoding

**Coset Structure:**
- Partition vector space into cosets of the code
- Each coset has unique syndrome
- Choose minimum weight error pattern as coset leader

**Decoding Table:**
```
Syndrome | Error Pattern
---------|---------------
000      | 000 (no error)
001      | 001
010      | 010
011      | 100
...      | ...
```

## Convolutional Codes

### Structure and Representation

**Convolutional Encoder:**
- Constraint Length: K (memory + 1)
- Rate: R = k/n (k input bits → n output bits per time unit)
- Generator Polynomials: G₁(D), G₂(D), ..., Gₙ(D)

**Polynomial Representation:**
For rate 1/2 code with polynomials (7, 5) in octal:
- G₁(D) = 1 + D + D² (octal 7 = 111₂)
- G₂(D) = 1 + D² (octal 5 = 101₂)

**State-Space Representation:**
- State: S_i = (u_{i-1}, u_{i-2}, ..., u_{i-K+2})
- State Transition: S_{i+1} = (u_i, u_{i-1}, ..., u_{i-K+3})
- Output: depends on current input and state

### Trellis Structure

**State Diagram:**
- 2^(K-1) states for binary codes
- Each state has 2 outgoing branches (for input 0 or 1)
- Branch labels show input/output pairs

**Trellis Sections:**
```
Time:     i-1        i        i+1
States:  00 -----> 00 -----> 00
         01 -----> 01 -----> 01
         10 -----> 10 -----> 10
         11 -----> 11 -----> 11
```

Each branch represents state transition with corresponding output bits.

### Distance Properties

**Free Distance (d_free):**
Minimum Hamming distance between any two distinct codeword sequences.

**For rate 1/2 codes:**
- K=3, (7,5): d_free = 5
- K=4, (15,17): d_free = 6
- K=5, (23,35): d_free = 7

**Error Correction Capability:**
Can correct up to ⌊(d_free - 1)/2⌋ errors.

### Transfer Function Analysis

**State Transition Matrix:**
```
T(D,N,J) = [T_ij(D,N,J)]
```
where:
- D: tracks Hamming weight
- N: tracks path length
- J: tracks number of nonzero information bits

**Transfer Function:**
```
T(D,N,J) = Σ A_d,l N^l D^d J^j
```

## Maximum Likelihood Decoding

### Theoretical Foundation

**Decision Rule:**
Choose codeword c̃ that maximizes P(r|c̃) where r is received sequence.

**For BSC (Binary Symmetric Channel):**
```
P(r|c) = p^{d_H(r,c)} (1-p)^{n-d_H(r,c)}
```

**Maximum Likelihood ≡ Minimum Distance:**
On BSC, ML decoding is equivalent to finding codeword closest in Hamming distance to received sequence.

### Viterbi Algorithm

**Dynamic Programming Approach:**
1. **Forward Pass**: Calculate cumulative metrics for all paths
2. **Path Storage**: Maintain best path to each state
3. **Traceback**: Follow best path to decode information sequence

**Path Metric Update:**
```
M_j^{(i)} = min_k [M_k^{(i-1)} + γ(k,j)]
```
where γ(k,j) is branch metric from state k to state j.

**Branch Metrics:**
For hard-decision decoding:
```
γ(k,j) = d_H(received_symbols, expected_symbols)
```

**Survivor Path:**
At each time step, keep only the best path to each state, discarding suboptimal paths.

### Soft-Decision Decoding

**Branch Metric Calculation:**
For received symbols r and expected symbols c:
```
γ = Σ_i r_i × c_i  (correlation metric)
```
or
```
γ = Σ_i (r_i - c_i)²  (Euclidean distance)
```

**Performance Gain:**
Soft-decision decoding typically provides 2-3 dB gain over hard-decision.

## Distance Properties

### Hamming Distance

**Definition:**
d_H(x,y) = number of positions where x_i ≠ y_i

**Properties:**
- Non-negative: d_H(x,y) ≥ 0
- Symmetric: d_H(x,y) = d_H(y,x)
- Triangle inequality: d_H(x,z) ≤ d_H(x,y) + d_H(y,z)

### Weight Distribution

**Weight Enumerator Polynomial:**
```
A(z) = Σ_{w=0}^n A_w z^w
```
where A_w is the number of codewords of weight w.

**MacWilliams Identity:**
For linear code with generator polynomial A(z):
```
B(z) = 2^{-k} (1+z)^n A((1-z)/(1+z))
```
where B(z) is weight enumerator of dual code.

### Error Detection and Correction

**Minimum Distance Decoding:**
- Decode to closest codeword
- If multiple codewords equidistant, declare error
- Guaranteed to correct ⌊(d_min-1)/2⌋ errors

**Bounded Distance Decoding:**
- Only decode if received word is within distance t of unique codeword
- Always correct when errors ≤ t
- May fail to decode correctable patterns beyond guaranteed distance

## Performance Analysis

### Error Probability Bounds

**Union Bound:**
For ML decoding on BSC with crossover probability p:
```
P_e ≤ Σ_{c≠c₀} P(c chosen | c₀ sent)
    ≤ Σ_{d=d_min}^n A_d × p^{d/2} (1-p)^{d/2}
```

**For Convolutional Codes:**
```
P_b ≤ (1/k) Σ_{d=d_free}^∞ β_d Q(√(2dR E_b/N_0))
```
where β_d is number of information bit errors on paths of weight d.

### Asymptotic Performance

**Error Exponent:**
For rates below capacity, error probability decreases exponentially with block length:
```
P_e ≤ 2^{-nE(R)}
```
where E(R) is the error exponent function.

**Cutoff Rate R_0:**
```
R_0 = 1 - log₂(1 + 2√(p(1-p)))
```

For rates R < R_0, simple sequential decoding algorithms exist.

## Channel Models

### Binary Symmetric Channel (BSC)

**Model:**
- Input: {0, 1}
- Output: {0, 1}
- Crossover probability: p
- P(Y=0|X=1) = P(Y=1|X=0) = p

**Capacity:**
```
C = 1 - H(p) = 1 + p log₂(p) + (1-p) log₂(1-p)
```

### Additive White Gaussian Noise (AWGN)

**Model:**
- Y = X + N where N ~ N(0, σ²)
- Continuous input/output

**Capacity (BPSK):**
```
C = (1/2) log₂(1 + 2E_b/N_0)
```

**Soft-Decision Metrics:**
- Receive continuous values
- Use likelihood ratios for decoding
- Better performance than hard decisions

### Fading Channels

**Rayleigh Fading:**
- Amplitude: |h|² ~ exponential distribution
- Phase: uniformly distributed
- Models urban mobile environment

**Performance:**
- Diversity techniques needed
- Coding provides time diversity
- Performance limited by fading statistics

## Practical Considerations

### Implementation Complexity

**Encoder Complexity:**
- Linear codes: O(nk) operations per codeword
- Convolutional codes: O(n) operations per information bit

**Decoder Complexity:**
- ML decoding: exponential in k (block codes) or K (convolutional)
- Syndrome decoding: O(n³) preprocessing, O(n) per codeword
- Viterbi decoding: O(2^K) per decoded bit

### Memory Requirements

**Viterbi Decoder:**
- Path storage: O(2^K × traceback_length)
- Metric storage: O(2^K)
- Total: dominated by traceback memory

**Block Code Decoder:**
- Syndrome table: O(2^{n-k})
- Or algebraic decoding: O(1) storage, higher computation

### Quantization Effects

**Soft-Decision Quantization:**
- 3-bit quantization captures most of soft-decision gain
- Beyond 4 bits provides diminishing returns
- Trade-off between performance and complexity

**Metric Scaling:**
- Prevent overflow in accumulated metrics
- Normalize by subtracting minimum metric periodically
- Use appropriate number format (fixed vs. floating point)

## Recent Developments

### Turbo Codes

**Parallel Concatenated Codes:**
- Two systematic convolutional codes
- Interleaver spreads error patterns
- Iterative decoding approaches ML performance

**Performance:**
- Near Shannon limit performance
- Practical for moderate to high SNR
- Used in 3G/4G cellular systems

### LDPC Codes

**Structure:**
- Sparse parity check matrices
- Tanner graph representation
- Belief propagation decoding

**Advantages:**
- Linear-time decoding
- Excellent performance with large block lengths
- Used in WiFi, WiMAX, DVB-S2

## Summary

Error correction coding provides the theoretical and practical foundation for reliable digital communication. The key insights are:

1. **Trade-offs**: Rate vs. error correction capability vs. complexity
2. **Optimality**: ML decoding is optimal but often impractical
3. **Structure**: Linear codes enable efficient encoding/decoding
4. **Performance**: Distance properties determine error correction capability
5. **Practical**: Implementation considerations drive algorithm choice

The implementations in CodeCrypt demonstrate these principles with practical, efficient algorithms suitable for educational and research purposes.

## See Also

- [Convolutional Codes Theory](convolutional-codes.md)
- [Viterbi Algorithm](viterbi-algorithm.md)
- [Turbo Codes Theory](turbo-codes.md)
- [API Documentation](../api/convolutional.md)