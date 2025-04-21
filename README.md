# CodeCrypt

A command-line tool for convolutional coding and basic cryptography, implemented in Rust.

## Overview

CodeCrypt is a simple CLI tool that demonstrates concepts from coding theory and basic cryptography. It implements:

1. **Convolutional Encoding and Decoding**: Implements convolutional encoding with configurable generator polynomials and constraint length, along with Viterbi decoding algorithm
2. **Turbo Coding**: A basic implementation of turbo codes, which use parallel concatenated convolutional codes
3. **Basic Cryptography**: Implementations of Caesar cipher, Vigenère cipher, and XOR encryption

## Installation

Make sure you have Rust and Cargo installed. Then:

```bash
# Clone the repository
git clone https://github.com/masterK0927/rustcrypto.git

# Build the project
cd codecrypt
cargo build --release

# Run the binary
./target/release/codecrypt --help
```

## Usage Examples

### Convolutional Encoding

```bash
# Encode "Hello" using constraint length 3 and polynomials 7,5 (in octal)
codecrypt conv-encode --input "Hello" --polynomials 7 5 --constraint-length 3
```

### Viterbi Decoding

```bash
# Decode convolutional code using the same parameters
codecrypt conv-decode --input "<encoded-data>" --polynomials 7 5 --constraint-length 3
```

### Turbo Encoding

```bash
# Encode "Hello" using turbo code with polynomials 7,5 (in octal)
codecrypt turbo-encode --input "Hello" --polynomials 7 5
```

### Encryption

```bash
# Encrypt "Hello, World!" using XOR with key "secret"
codecrypt encrypt --input "Hello, World!" --key "secret" --algorithm xor

# Encrypt "Hello, World!" using Caesar cipher with shift 3
codecrypt encrypt --input "Hello, World!" --key "3" --algorithm caesar

# Encrypt "Hello, World!" using Vigenère cipher with key "KEY"
codecrypt encrypt --input "Hello, World!" --key "KEY" --algorithm vigenere
```

### Decryption

```bash
# Decrypt using XOR with key "secret"
codecrypt decrypt --input "<encrypted-data>" --key "secret" --algorithm xor
```

## Concepts and Theory

### Convolutional Codes

Convolutional codes are a type of error-correcting code where:
- Each output bit depends on K previous input bits (K = constraint length)
- They are defined by generator polynomials which specify connections to modulo-2 adders
- They provide good error correction capabilities for communication channels

### Viterbi Decoding

The Viterbi algorithm is an optimal decoding method for convolutional codes:
- Uses a trellis structure to represent all possible encoder states
- Finds the maximum likelihood path through the trellis
- Efficient implementation with computational complexity linear in the message length

### Turbo Codes

Turbo codes are powerful error correction codes that:
- Use parallel concatenated convolutional codes
- Include an interleaver to reduce correlation between redundant bits
- Can approach the Shannon limit (theoretical maximum capacity of a channel)

### Basic Cryptography

The tool implements three simple encryption methods:
- **Caesar Cipher**: Shifts each letter by a fixed amount
- **Vigenère Cipher**: Uses a keyword to determine variable shifts for letters
- **XOR Cipher**: Applies bitwise XOR operation with a key

## License

This project is licensed under the MIT License - see the LICENSE file for details.