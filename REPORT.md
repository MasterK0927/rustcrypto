# CodeCrypt Project Presentation

## Project Overview

CodeCrypt is a command-line interface (CLI) tool written in Rust that demonstrates fundamental concepts from coding theory and basic cryptography. It's designed as a simple, educational tool to help understand and experiment with:

1. Convolutional codes and Viterbi decoding
2. Turbo codes (basic implementation)
3. Simple encryption algorithms

## Motivation

In communication systems, we often need to transmit data over noisy channels while maintaining integrity. Similarly, we sometimes need to protect data from unauthorized access. This project combines both aspects in a single, easy-to-use tool.

## Key Concepts Explained

### Convolutional Codes

Convolutional codes are a class of error-correcting codes that:
- Process information sequentially
- Generate output based on current and previous input bits
- Are defined by generator polynomials and constraint length

#### Polynomial Description

A convolutional code is defined by:
- **Constraint Length (K)**: The number of bits in the encoder memory plus one
- **Generator Polynomials**: Define connections to modulo-2 adders

For example, a (2,1,3) convolutional code with generators [7,5] in octal:
- Takes 1 bit as input
- Produces 2 bits as output
- Has constraint length 3
- Uses polynomials 111 (7 in octal) and 101 (5 in octal)

#### Matrix Description

Convolutional codes can also be described using generator matrices, which represent the encoder's impulse response. This matrix format helps in mathematical analysis.

### Viterbi Decoding

The Viterbi algorithm:
- Is an optimal maximum-likelihood sequence estimation (MLSE) decoder
- Uses dynamic programming to efficiently find the most likely transmitted sequence
- Works by tracking path metrics through a trellis representation

### Turbo Codes

Turbo codes:
- Consist of two parallel concatenated convolutional encoders
- Include an interleaver between encoders to reduce correlation
- Approach channel capacity (Shannon limit)
- Use iterative decoding with soft-decision information

### Cryptography Overview

The project implements three classic encryption techniques:

1. **Caesar Cipher**: A substitution cipher that shifts letters by a fixed amount
   - Simple but easily broken
   - Historical significance (used by Julius Caesar)

2. **Vigenère Cipher**: A polyalphabetic substitution cipher
   - Uses a keyword to determine variable shifts
   - Resists simple frequency analysis

3. **XOR Cipher**: A symmetric encryption algorithm
   - Uses the XOR operation which is its own inverse
   - Simple yet powerful when used properly

## Implementation Details

### Architecture

The project follows a modular architecture:
- `convolutional`: Contains encoder and Viterbi decoder
- `turbo`: Implements turbo encoding
- `crypto`: Implements various cipher algorithms
- `cli`: Handles command-line interface and user inputs

### Key Algorithms

#### Convolutional Encoding
```rust
// For each input bit:
// 1. Shift it into the register
// 2. Apply each generator polynomial
// 3. Calculate parity for each polynomial
```

#### Viterbi Decoding
```rust
// 1. Initialize path metrics
// 2. For each received symbol:
//    a. Consider all current states
//    b. Calculate branch metrics to next states
//    c. Update best paths
// 3. Select path with minimum metric
```

#### Turbo Encoding
```rust
// 1. Encode input using first encoder
// 2. Interleave input
// 3. Encode interleaved input using second encoder
// 4. Combine outputs (systematic + parity bits)
```

## Demo Usage Examples

### 1. Encoding a message with a convolutional code

```bash
$ codecrypt conv-encode --input "Hello" --polynomials 7 5 --constraint-length 3
```

### 2. Decoding using Viterbi algorithm

```bash
$ codecrypt conv-decode --input "<encoded-data>" --polynomials 7 5 --constraint-length 3
```

### 3. Encrypting a message

```bash
$ codecrypt encrypt --input "Hello, World!" --key "SECRET" --algorithm vigenere
```

## Practical Applications

1. **Communications**: Error correction codes are essential in:
   - Satellite communications
   - Mobile networks
   - Digital broadcasting
   - Deep space communications

2. **Security**: Basic encryption techniques demonstrate:
   - Principles of information security
   - Foundation for modern cryptography
   - Trade-offs between complexity and security

## Limitations and Future Work

### Current Limitations
- Basic implementation without optimization
- Limited support for code rates and parameters
- Simplified turbo code without iterative decoding
- Educational cryptography not suitable for production use

### Potential Enhancements
- Add support for puncturing to achieve different code rates
- Implement iterative decoding for turbo codes
- Add visualization of trellis and decoding process
- Include more modern encryption algorithms

## Conclusion

CodeCrypt demonstrates the fundamental principles of error correction coding and basic cryptography in a simple Rust CLI tool. It provides a practical way to understand these concepts through hands-on experimentation.

The project balances theoretical correctness with practical simplicity, making it suitable for educational purposes and further expansion.