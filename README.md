# CodeCrypt

A comprehensive CLI tool for convolutional coding and cryptography, implemented in Rust. This project provides implementations of forward error correction (FEC) algorithms and various cryptographic ciphers.

## Features

### Error Correction Coding
- **Convolutional Encoding**: Implement convolutional codes with configurable constraint lengths and generator polynomials
- **Viterbi Decoding**: Maximum likelihood decoding using the Viterbi algorithm
- **Turbo Encoding**: Advanced error correction using parallel concatenated convolutional codes

### Cryptographic Algorithms
- **Caesar Cipher**: Simple substitution cipher with configurable shift values
- **Vigenère Cipher**: Polyalphabetic substitution cipher using keyword-based encryption
- **XOR Cipher**: Stream cipher using XOR operations with a repeating key

## Table of Contents

- [CodeCrypt](#codecrypt)
  - [Features](#features)
    - [Error Correction Coding](#error-correction-coding)
    - [Cryptographic Algorithms](#cryptographic-algorithms)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
    - [Prerequisites](#prerequisites)
    - [Build from Source](#build-from-source)
  - [Usage](#usage)
    - [Convolutional Encoding](#convolutional-encoding)
    - [Convolutional Decoding](#convolutional-decoding)
    - [Turbo Encoding](#turbo-encoding)
    - [Encryption](#encryption)
    - [Decryption](#decryption)
  - [🔬 Implementation Details](#-implementation-details)
    - [Convolutional Encoder](#convolutional-encoder)
    - [Viterbi Decoder](#viterbi-decoder)
    - [Turbo Encoder](#turbo-encoder)
    - [Cipher Implementations](#cipher-implementations)
      - [Caesar Cipher](#caesar-cipher)
      - [Vigenère Cipher](#vigenère-cipher)
      - [XOR Cipher](#xor-cipher)
  - [🔗 Examples](#-examples)
    - [Complete Workflow Example](#complete-workflow-example)
    - [Polynomial Configuration Examples](#polynomial-configuration-examples)
  - [🧪 Testing](#-testing)
  - [Dependencies](#dependencies)
  - [Project Structure](#project-structure)
  - [Development](#development)
    - [Code Style](#code-style)
    - [Adding New Features](#adding-new-features)
    - [Performance Considerations](#performance-considerations)
  - [Contributing](#contributing)
    - [Coding Standards](#coding-standards)
  - [Further Reading](#further-reading)

## Installation

### Prerequisites
- Rust 1.70.0 or later
- Cargo package manager

### Build from Source

```bash
git clone https://github.com/yourusername/codecrypt.git
cd codecrypt
cargo build --release
```

The binary will be available at `target/release/codecrypt`

## Usage

CodeCrypt provides a command-line interface with several subcommands for different operations.

### Convolutional Encoding

Encode a message using convolutional codes:

```bash
codecrypt conv-encode -i "Hello" -p 7 5 -c 3
```

**Parameters:**
- `-i, --input`: Input message to encode
- `-p, --polynomials`: Generator polynomials in octal notation (e.g., 7 5 for polynomials 111₂ and 101₂)
- `-c, --constraint-length`: Constraint length (default: 3)

### Convolutional Decoding

Decode a convolutional encoded message using the Viterbi algorithm:

```bash
codecrypt conv-decode -i "[encoded_message]" -p 7 5 -c 3
```

**Parameters:**
- `-i, --input`: Encoded message to decode
- `-p, --polynomials`: Generator polynomials (must match encoding parameters)
- `-c, --constraint-length`: Constraint length (must match encoding parameters)

### Turbo Encoding

Encode a message using turbo codes:

```bash
codecrypt turbo-encode -i "Hello" -p 7 5
```

**Parameters:**
- `-i, --input`: Input message to encode
- `-p, --polynomials`: Generator polynomials for the constituent encoders

### Encryption

Encrypt a message using various cipher algorithms:

```bash
# Caesar cipher
codecrypt encrypt -i "Hello World" -k "3" -a caesar

# Vigenère cipher
codecrypt encrypt -i "Hello World" -k "secret" -a vigenere

# XOR cipher
codecrypt encrypt -i "Hello World" -k "key" -a xor
```

**Parameters:**
- `-i, --input`: Message to encrypt
- `-k, --key`: Encryption key
- `-a, --algorithm`: Cipher algorithm (caesar, vigenere, xor) [default: xor]

### Decryption

Decrypt a message:

```bash
codecrypt decrypt -i "[encrypted_message]" -k "key" -a algorithm
```

**Parameters:**
- `-i, --input`: Encrypted message to decrypt
- `-k, --key`: Decryption key (must match encryption key)
- `-a, --algorithm`: Cipher algorithm used for encryption

## 🔬 Implementation Details

### Convolutional Encoder

The convolutional encoder implements a rate-1/n systematic convolutional code with the following features:

- **Constraint Length**: Defines the memory of the encoder (K)
- **Generator Polynomials**: Octal representation converted to binary for XOR operations
- **State Management**: Maintains encoder state across input bits
- **Output Generation**: Produces n output bits for each input bit

**Key Implementation (`src/convolutional/encoder.rs:54-75`):**
```rust
pub fn encode(&mut self, input: &[bool]) -> Vec<bool> {
    let register_mask = (1 << (self.constraint_length - 1)) - 1;
    let mut output = Vec::with_capacity(input.len() * self.rate_denominator);

    for &bit in input {
        self.state = ((self.state << 1) | (if bit { 1 } else { 0 })) & register_mask;

        for &poly in &self.polynomials {
            let mut parity = 0;
            let mut temp = self.state & poly;

            while temp != 0 {
                parity ^= temp & 1;
                temp >>= 1;
            }

            output.push(parity == 1);
        }
    }

    output
}
```

### Viterbi Decoder

The Viterbi decoder implements maximum likelihood sequence estimation:

- **Trellis Structure**: Represents all possible encoder states and transitions
- **Path Metrics**: Maintains accumulated Hamming distances for each path
- **Branch Metrics**: Calculates distance between received and expected symbols
- **Traceback**: Selects the path with minimum accumulated metric

**Algorithm Features:**
- Handles arbitrary rate-1/n codes
- Soft-decision compatible (currently implements hard-decision)
- Memory-efficient path storage using HashMap

### Turbo Encoder

The turbo encoder implements parallel concatenated convolutional codes:

- **Systematic Output**: Original input bits are transmitted unchanged
- **Parallel Encoders**: Two identical convolutional encoders process the input
- **Interleaving**: Second encoder processes a permuted version of the input
- **Output Format**: [systematic bits, parity1, parity2]

**Interleaver Design (`src/turbo/encoder.rs:35-45`):**
```rust
fn interleave(&self, input: &[bool]) -> Vec<bool> {
    let len = input.len();

    let mut interleaved = vec![false; len];
    for i in 0..len {
        let new_pos = (i * 7 + 5) % len;  // Simple algebraic interleaver
        interleaved[new_pos] = input[i];
    }
    interleaved
}
```

### Cipher Implementations

#### Caesar Cipher
- **Algorithm**: Simple substitution with fixed shift
- **Key Format**: Integer shift value (0-25)
- **Features**: Preserves case, ignores non-alphabetic characters

#### Vigenère Cipher
- **Algorithm**: Polyalphabetic substitution using repeating keyword
- **Key Format**: Alphabetic string (case-insensitive)
- **Features**: Keyword repetition, case preservation

#### XOR Cipher
- **Algorithm**: Bitwise XOR with repeating key
- **Key Format**: Any string
- **Output**: Hexadecimal encoded result
- **Features**: Self-inverse operation (encrypt = decrypt)

**XOR Implementation (`src/crypto/cipher.rs:136-148`):**
```rust
fn xor_cipher(&self, message: &str) -> String {
    let key_bytes = self.key.as_bytes();

    if key_bytes.is_empty() {
        return message.to_string();
    }

    message.bytes()
        .enumerate()
        .map(|(i, b)| format!("{:02x}", b ^ key_bytes[i % key_bytes.len()]))
        .collect::<Vec<_>>()
        .join("")
}
```

## 🔗 Examples

### Complete Workflow Example

```bash
# 1. Encode a message with convolutional code
codecrypt conv-encode -i "Test" -p 7 5 -c 3

# 2. Decode the message (using output from step 1)
codecrypt conv-decode -i "[encoded_output]" -p 7 5 -c 3

# 3. Encrypt with different algorithms
codecrypt encrypt -i "Secret Message" -k "mykey" -a vigenere
codecrypt encrypt -i "Secret Message" -k "5" -a caesar
codecrypt encrypt -i "Secret Message" -k "password" -a xor

# 4. Turbo encoding for better error correction
codecrypt turbo-encode -i "Important Data" -p 7 5
```

### Polynomial Configuration Examples

Common generator polynomials for different constraint lengths:

- **K=3**: (7,5) → (111₂, 101₂)
- **K=4**: (15,17) → (1101₂, 1111₂)
- **K=5**: (23,35) → (10011₂, 11101₂)

```bash
# Rate 1/2 code with constraint length 4
codecrypt conv-encode -i "Hello" -p 15 17 -c 4

# Rate 1/3 code with constraint length 3
codecrypt conv-encode -i "Hello" -p 7 5 3 -c 3
```

## 🧪 Testing

The project includes comprehensive unit tests for all components:

```bash
# Run all tests
cargo test

# Run tests for specific modules
cargo test convolutional
cargo test crypto
cargo test turbo

# Run tests with output
cargo test -- --nocapture
```

**Test Coverage:**
- Convolutional encoder/decoder round-trip tests
- Cipher encrypt/decrypt verification
- Turbo encoder systematic bit verification
- Error handling and edge cases

## Dependencies

The project uses the following Rust crates:

- **clap** (4.4+): Command-line argument parsing with derive macros
- **bitvec** (1.0+): Efficient bit vector operations
- **rand** (0.8+): Random number generation for testing

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }
bitvec = "1.0"
rand = "0.8"
```

## Project Structure

```
src/
├── main.rs              # Main entry point and command routing
├── cli.rs               # Command-line interface definitions
├── convolutional/       # Convolutional coding module
│   ├── mod.rs
│   ├── encoder.rs       # Convolutional encoder implementation
│   └── viterbi.rs       # Viterbi decoder implementation
├── turbo/              # Turbo coding module
│   ├── mod.rs
│   └── encoder.rs       # Turbo encoder implementation
└── crypto/             # Cryptographic algorithms
    ├── mod.rs
    └── cipher.rs        # Cipher implementations
```

## Development

### Code Style
- Follow Rust standard formatting with `cargo fmt`
- Use `cargo clippy` for linting
- Maintain comprehensive documentation

### Adding New Features

1. **New Cipher**: Add implementation to `src/crypto/cipher.rs`
2. **New Coding Algorithm**: Create new module under `src/`
3. **CLI Commands**: Update `src/cli.rs` and `src/main.rs`

### Performance Considerations

- Bit operations are optimized for performance
- Memory allocation is minimized in encoding/decoding loops
- Hash maps provide efficient state management in Viterbi algorithm

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Coding Standards
- Write comprehensive tests for new functionality
- Document all public APIs
- Follow existing code patterns and naming conventions
- Ensure backward compatibility

## Further Reading

- **Error Correction Coding**: "Error Control Coding" by Lin & Costello
- **Cryptography**: "Applied Cryptography" by Bruce Schneier
- **Digital Communications**: "Digital Communications" by Proakis & Salehi