# Getting Started with CodeCrypt

This tutorial will get you up and running with CodeCrypt quickly, covering installation, basic usage, and your first encoding/encryption operations.

## Prerequisites

Before you begin, make sure you have:
- Rust 1.70.0 or later installed ([Install Rust](https://rustup.rs/))
- Basic familiarity with command-line interfaces
- Text editor of your choice

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/codecrypt.git
cd codecrypt
```

### 2. Build the Project

```bash
# Build in release mode for optimal performance
cargo build --release

# The binary will be available at:
# target/release/codecrypt
```

### 3. Verify Installation

```bash
# Test the installation
./target/release/codecrypt --help
```

You should see the help message with all available commands.

### 4. Optional: Install Globally

```bash
# Install to ~/.cargo/bin (make sure it's in your PATH)
cargo install --path .

# Now you can use 'codecrypt' from anywhere
codecrypt --help
```

## Your First Commands

Let's start with simple examples to understand how CodeCrypt works.

### Basic Encryption

#### Caesar Cipher
The simplest cipher - shifts each letter by a fixed number of positions:

```bash
# Encrypt a message with Caesar cipher (shift by 3)
codecrypt encrypt -i "Hello World" -k "3" -a caesar
```

**Output:**
```
Encrypted: Khoor Zruog
```

#### Decrypt the Message
```bash
# Decrypt using the same key
codecrypt decrypt -i "Khoor Zruog" -k "3" -a caesar
```

**Output:**
```
Decrypted: Hello World
```

### Advanced Encryption

#### Vigenère Cipher
Uses a keyword to create a more secure encryption:

```bash
# Encrypt with Vigenère cipher
codecrypt encrypt -i "Secret Message" -k "password" -a vigenere
```

**Example output:**
```
Encrypted: Hqjkkt Qbacudc
```

#### XOR Cipher
Binary encryption that's self-reversing:

```bash
# Encrypt with XOR
codecrypt encrypt -i "Top Secret" -k "mykey" -a xor
```

**Example output:**
```
Encrypted: 17503b1c051b491c4b
```

```bash
# Decrypt (same operation)
codecrypt decrypt -i "17503b1c051b491c4b" -k "mykey" -a xor
```

### Error Correction Coding

#### Convolutional Encoding
Adds redundancy to protect against transmission errors:

```bash
# Encode with convolutional code
codecrypt conv-encode -i "Data" -p 7 5 -c 3
```

**Parameters explained:**
- `-i "Data"`: Input message
- `-p 7 5`: Generator polynomials (octal notation)
- `-c 3`: Constraint length

#### Convolutional Decoding
Recover original data using Viterbi algorithm:

```bash
# First encode some data
ENCODED=$(codecrypt conv-encode -i "Test" -p 7 5 -c 3)

# Then decode it back
codecrypt conv-decode -i "$ENCODED" -p 7 5 -c 3
```

#### Turbo Encoding
Advanced error correction for harsh environments:

```bash
# Turbo encode for maximum protection
codecrypt turbo-encode -i "Critical Data" -p 7 5
```

## Understanding the Output

### Text vs Binary Output

**Text Ciphers (Caesar, Vigenère):**
- Input: Regular text
- Output: Transformed text
- Readable but encrypted

**Binary Ciphers (XOR):**
- Input: Any text
- Output: Hexadecimal string
- Not human-readable

**Error Correction:**
- Input: Text (converted to bits internally)
- Output: Encoded bit stream (displayed as text)

### Working with Different Data Types

#### Text Messages
```bash
# Simple text
codecrypt encrypt -i "Hello" -k "secret" -a vigenere

# Text with punctuation
codecrypt encrypt -i "Hello, World!" -k "key" -a caesar

# Longer messages
codecrypt encrypt -i "This is a longer message to demonstrate the cipher" -k "longkey" -a vigenere
```

#### Binary Data (via XOR)
```bash
# Any characters work with XOR
codecrypt encrypt -i "Data123!@#" -k "binarykey" -a xor
```

## Common Patterns and Workflows

### 1. Testing Round-trip Encryption

```bash
#!/bin/bash
# test-encryption.sh

MESSAGE="Test message for encryption"
KEY="testkey"

# Test each algorithm
for ALGO in caesar vigenere xor; do
    echo "Testing $ALGO cipher:"

    # Encrypt
    ENCRYPTED=$(codecrypt encrypt -i "$MESSAGE" -k "$KEY" -a "$ALGO")
    echo "  Encrypted: $ENCRYPTED"

    # Decrypt
    DECRYPTED=$(codecrypt decrypt -i "$ENCRYPTED" -k "$KEY" -a "$ALGO")
    echo "  Decrypted: $DECRYPTED"

    # Verify
    if [ "$MESSAGE" = "$DECRYPTED" ]; then
        echo "  ✅ Round-trip successful"
    else
        echo "  ❌ Round-trip failed"
    fi
    echo
done
```

### 2. Testing Error Correction

```bash
#!/bin/bash
# test-error-correction.sh

MESSAGE="Error correction test"
POLYNOMIALS="7 5"
CONSTRAINT_LENGTH="3"

echo "Testing convolutional coding:"
echo "Original: $MESSAGE"

# Encode
ENCODED=$(codecrypt conv-encode -i "$MESSAGE" -p $POLYNOMIALS -c $CONSTRAINT_LENGTH)
echo "Encoded: $ENCODED"

# Decode
DECODED=$(codecrypt conv-decode -i "$ENCODED" -p $POLYNOMIALS -c $CONSTRAINT_LENGTH)
echo "Decoded: $DECODED"

# Verify
if [ "$MESSAGE" = "$DECODED" ]; then
    echo "✅ Error correction successful"
else
    echo "❌ Error correction failed"
fi
```

### 3. Comparing Algorithm Strengths

```bash
#!/bin/bash
# compare-algorithms.sh

MESSAGE="Compare algorithm security"

echo "Comparing encryption algorithms:"
echo "Original: $MESSAGE"
echo

# Caesar (weak)
CAESAR=$(codecrypt encrypt -i "$MESSAGE" -k "5" -a caesar)
echo "Caesar (shift 5):  $CAESAR"

# Vigenère (stronger)
VIGENERE=$(codecrypt encrypt -i "$MESSAGE" -k "secret" -a vigenere)
echo "Vigenère (secret): $VIGENERE"

# XOR (different output format)
XOR=$(codecrypt encrypt -i "$MESSAGE" -k "secret" -a xor)
echo "XOR (secret):      $XOR"
```

## Configuration and Customization

### Environment Variables

You can set up aliases or environment variables for commonly used parameters:

```bash
# Add to your ~/.bashrc or ~/.zshrc
export CODECRYPT_PATH="./target/release/codecrypt"
alias cc="$CODECRYPT_PATH"

# Now you can use shorter commands
cc encrypt -i "message" -k "key" -a vigenere
```

### Creating Wrapper Scripts

Create custom scripts for specific use cases:

```bash
#!/bin/bash
# secure-encrypt.sh - Always use Vigenère with strong key

if [ $# -ne 2 ]; then
    echo "Usage: $0 <message> <key>"
    exit 1
fi

codecrypt encrypt -i "$1" -k "$2" -a vigenere
```

## Troubleshooting

### Common Issues

#### 1. Command Not Found
```bash
# If you get "codecrypt: command not found"
# Use full path:
./target/release/codecrypt --help

# Or add to PATH:
export PATH=$PATH:$(pwd)/target/release
```

#### 2. Build Errors
```bash
# Update Rust if you get compilation errors
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

#### 3. Invalid Parameters
```bash
# For convolutional codes, polynomials must be valid
# ❌ Wrong: codecrypt conv-encode -i "test" -p 999 -c 3
# ✅ Correct: codecrypt conv-encode -i "test" -p 7 5 -c 3
```

#### 4. Decryption Failures
```bash
# Make sure parameters match exactly
# Encrypt:
codecrypt encrypt -i "test" -k "key123" -a vigenere

# Decrypt (must use same key and algorithm):
codecrypt decrypt -i "[encrypted]" -k "key123" -a vigenere
```

### Getting Help

```bash
# General help
codecrypt --help

# Command-specific help
codecrypt encrypt --help
codecrypt conv-encode --help
codecrypt turbo-encode --help
```

## Next Steps

Now that you have CodeCrypt running, you can:

1. **Learn the Algorithms**: Read the [Theory Documentation](../theory/) to understand how the algorithms work
2. **Try Advanced Features**: Explore [Advanced Tutorial](advanced-usage.md)
3. **Understand the Code**: Check the [API Documentation](../api/) for implementation details
4. **Contribute**: See the [Contributing Guide](../development/contributing.md)

### Recommended Learning Path

1. **Start Here**: Basic encryption with Caesar and Vigenère ciphers
2. **Explore XOR**: Understand binary encryption and hex output
3. **Error Correction**: Learn convolutional coding for data protection
4. **Advanced Topics**: Turbo codes and algorithm comparison
5. **Theory Deep-dive**: Mathematical foundations and security analysis

### Practice Exercises

Try these challenges to reinforce your learning:

1. **Caesar Challenge**: Decrypt this message (shift unknown): "Wklv lv d vhfuhw phvvdjh"
2. **Vigenère Challenge**: Find the key length for a long encrypted text
3. **XOR Challenge**: Recover a message where you know part of the plaintext
4. **Error Correction**: Compare output sizes for different coding rates

## Resources

- [Full Documentation](../README.md)
- [API Reference](../api/)
- [Algorithm Theory](../theory/)
- [Examples](../examples/)
- [Source Code](https://github.com/yourusername/codecrypt)

Welcome to CodeCrypt! You're ready to explore the fascinating world of cryptography and error correction coding.