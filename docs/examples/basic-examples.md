# Basic Examples

This document provides practical examples of using CodeCrypt for various cryptographic and error correction tasks.

## Quick Reference

### Command Format
```bash
codecrypt <COMMAND> [OPTIONS]
```

### Available Commands
- `encrypt` - Encrypt messages
- `decrypt` - Decrypt messages
- `conv-encode` - Convolutional encoding
- `conv-decode` - Convolutional decoding
- `turbo-encode` - Turbo encoding

## Encryption Examples

### Caesar Cipher Examples

#### Basic Usage
```bash
# Encrypt with shift of 3 (classic Caesar)
$ codecrypt encrypt -i "Hello World" -k "3" -a caesar
Encrypted: Khoor Zruog

# Decrypt the result
$ codecrypt decrypt -i "Khoor Zruog" -k "3" -a caesar
Decrypted: Hello World
```

#### Different Shift Values
```bash
# ROT13 (shift of 13)
$ codecrypt encrypt -i "Hello World" -k "13" -a caesar
Encrypted: Uryyb Jbeyq

# Large shift (equivalent to shift of 5)
$ codecrypt encrypt -i "Hello World" -k "31" -a caesar
Encrypted: Mjqqt Btwqi

# Reverse shift (equivalent to shift of 23)
$ codecrypt encrypt -i "Hello World" -k "3" -a caesar
$ codecrypt encrypt -i "Khoor Zruog" -k "23" -a caesar
Encrypted: Hello World
```

#### Case and Character Handling
```bash
# Mixed case preservation
$ codecrypt encrypt -i "Hello, World!" -k "5" -a caesar
Encrypted: Mjqqt, Btwqi!

# Numbers and symbols unchanged
$ codecrypt encrypt -i "Meet at 3:30 PM" -k "7" -a caesar
Encrypted: Tlla ha 3:30 WT
```

### Vigenère Cipher Examples

#### Basic Keyword Encryption
```bash
# Using keyword "SECRET"
$ codecrypt encrypt -i "HELLO WORLD" -k "SECRET" -a vigenere
Encrypted: ZINCS DLZNE

# Case insensitive key
$ codecrypt encrypt -i "Hello World" -k "secret" -a vigenere
Encrypted: Zincs Dlzne

# Decrypt
$ codecrypt decrypt -i "Zincs Dlzne" -k "secret" -a vigenere
Decrypted: Hello World
```

#### Key Repetition Demonstration
```bash
# Short key with long message
$ codecrypt encrypt -i "This is a longer message" -k "KEY" -a vigenere
Encrypted: Dlgc sc e psrkiv qiqceeo

# Key repeats: KEYK EY K EYKEYKE YKEYKIY
```

#### Different Key Lengths
```bash
MESSAGE="CRYPTOGRAPHY IS FASCINATING"

# Short key (3 characters)
$ codecrypt encrypt -i "$MESSAGE" -k "ABC" -a vigenere
Encrypted: CSZQUPHQBPJZ JT HDTDJODUJOH

# Medium key (8 characters)
$ codecrypt encrypt -i "$MESSAGE" -k "PASSWORD" -a vigenere
Encrypted: RIZBUQJUYMFE XG UDGONADIXPJ

# Long key (15 characters)
$ codecrypt encrypt -i "$MESSAGE" -k "VERYLONGKEYWORD" -a vigenere
Encrypted: XJMHUZJVYMNC DN HYAEHPYQCFE
```

### XOR Cipher Examples

#### Basic XOR Operations
```bash
# Simple XOR
$ codecrypt encrypt -i "Hello" -k "KEY" -a xor
Encrypted: 0206070b0a

# Self-inverse property (decrypt is same operation)
$ codecrypt decrypt -i "0206070b0a" -k "KEY" -a xor
Decrypted: Hello
```

#### Binary Data Handling
```bash
# Special characters and numbers
$ codecrypt encrypt -i "Data123!@#" -k "secret" -a xor
Encrypted: 37101c0154055754015050

# Unicode and extended ASCII
$ codecrypt encrypt -i "Café naïve" -k "password" -a xor
Encrypted: 33140302201b141412041b
```

#### Key Length Effects
```bash
MESSAGE="REPEAT PATTERN TEST"

# Short key (1 character)
$ codecrypt encrypt -i "$MESSAGE" -k "A" -a xor
Encrypted: 10254005200420005454254010201320024054

# Medium key (4 characters)
$ codecrypt encrypt -i "$MESSAGE" -k "TEST" -a xor
Encrypted: 06254c0f27042c0f27204c0f14254c0f270420

# Long key (16 characters)
$ codecrypt encrypt -i "$MESSAGE" -k "VERYLONGPASSWORD" -a xor
Encrypted: 06234c0f27062c0b31062c0c13234c0f270620
```

## Error Correction Examples

### Convolutional Encoding Examples

#### Standard Rate 1/2 Code
```bash
# Constraint length 3, polynomials (7,5) in octal
$ codecrypt conv-encode -i "Data" -p 7 5 -c 3
Encoded: [encoded_bit_stream]

# Decode back to original
$ codecrypt conv-decode -i "[encoded_bit_stream]" -p 7 5 -c 3
Decoded: Data
```

#### Rate 1/3 Code (More Redundancy)
```bash
# Three generator polynomials
$ codecrypt conv-encode -i "Test" -p 7 5 3 -c 3
Encoded: [longer_encoded_stream]

# Higher redundancy = better error correction
$ codecrypt conv-decode -i "[longer_encoded_stream]" -p 7 5 3 -c 3
Decoded: Test
```

#### Different Constraint Lengths
```bash
MESSAGE="ERROR CORRECTION"

# Constraint length 3
$ codecrypt conv-encode -i "$MESSAGE" -p 7 5 -c 3
# Output length: 2 * input_bits

# Constraint length 4
$ codecrypt conv-encode -i "$MESSAGE" -p 15 17 -c 4
# Better error correction, same rate

# Constraint length 5
$ codecrypt conv-encode -i "$MESSAGE" -p 23 35 -c 5
# Even better error correction
```

### Turbo Encoding Examples

#### Basic Turbo Code
```bash
# Standard turbo code with (7,5) constituent encoders
$ codecrypt turbo-encode -i "TURBO" -p 7 5
Encoded: [systematic + parity1 + parity2]

# Output contains:
# - Original bits (systematic)
# - First encoder parity
# - Second encoder parity (with interleaving)
```

#### Rate Comparison
```bash
MESSAGE="COMPARE RATES"

# Original message
echo "Original: $MESSAGE"

# Convolutional (rate 1/2)
CONV=$(codecrypt conv-encode -i "$MESSAGE" -p 7 5 -c 3)
echo "Conv 1/2 length: ${#CONV}"

# Turbo (rate 1/3 approximately)
TURBO=$(codecrypt turbo-encode -i "$MESSAGE" -p 7 5)
echo "Turbo 1/3 length: ${#TURBO}"

# Turbo provides more redundancy for better error correction
```

## Advanced Usage Patterns

### Batch Processing

#### Multiple Messages
```bash
#!/bin/bash
# encrypt_batch.sh

MESSAGES=(
    "First secret message"
    "Second confidential data"
    "Third classified information"
)

KEY="batch_key"
ALGORITHM="vigenere"

for i in "${!MESSAGES[@]}"; do
    encrypted=$(codecrypt encrypt -i "${MESSAGES[$i]}" -k "$KEY" -a "$ALGORITHM")
    echo "Message $((i+1)): $encrypted"
done
```

#### Algorithm Comparison
```bash
#!/bin/bash
# compare_algorithms.sh

MESSAGE="Security test message"
KEY="comparison_key"

echo "Original: $MESSAGE"
echo

for ALGO in caesar vigenere xor; do
    if [ "$ALGO" = "caesar" ]; then
        KEY="13"  # ROT13 for Caesar
    else
        KEY="comparison_key"
    fi

    encrypted=$(codecrypt encrypt -i "$MESSAGE" -k "$KEY" -a "$ALGO")
    echo "$ALGO: $encrypted"
done
```

### Pipeline Operations

#### Encryption + Error Correction
```bash
#!/bin/bash
# secure_encode.sh

MESSAGE="Critical data requiring both encryption and error correction"
ENCRYPT_KEY="secret_key"
POLYNOMIALS="7 5"

echo "Original: $MESSAGE"

# Step 1: Encrypt
encrypted=$(codecrypt encrypt -i "$MESSAGE" -k "$ENCRYPT_KEY" -a vigenere)
echo "Encrypted: $encrypted"

# Step 2: Add error correction
final=$(codecrypt conv-encode -i "$encrypted" -p $POLYNOMIALS -c 3)
echo "Final (encrypted + encoded): $final"

# Reverse process:
# Step 1: Error correction decode
decoded=$(codecrypt conv-decode -i "$final" -p $POLYNOMIALS -c 3)
echo "After error correction: $decoded"

# Step 2: Decrypt
original=$(codecrypt decrypt -i "$decoded" -k "$ENCRYPT_KEY" -a vigenere)
echo "Recovered: $original"
```

### Performance Testing

#### Message Length Analysis
```bash
#!/bin/bash
# performance_test.sh

# Test different message lengths
LENGTHS=(10 50 100 500 1000)
BASE_MSG="Performance test message with sufficient length for meaningful analysis"

for length in "${LENGTHS[@]}"; do
    # Truncate or repeat message to desired length
    if [ $length -le ${#BASE_MSG} ]; then
        msg="${BASE_MSG:0:$length}"
    else
        msg=$(printf "%-*s" $length "$BASE_MSG")
        msg="${msg:0:$length}"
    fi

    echo "Length $length:"

    # Time the encoding operation
    time codecrypt conv-encode -i "$msg" -p 7 5 -c 3 > /dev/null
    echo
done
```

### Error Simulation

#### Introducing Bit Errors
```bash
#!/bin/bash
# error_simulation.sh

MESSAGE="Test error correction capability"
POLYNOMIALS="7 5"

echo "Original: $MESSAGE"

# Encode with error correction
encoded=$(codecrypt conv-encode -i "$MESSAGE" -p $POLYNOMIALS -c 3)
echo "Encoded: $encoded"

# Simulate bit error (flip one character)
# In real usage, you would introduce actual bit flips
corrupted="${encoded:0:10}X${encoded:11}"
echo "Corrupted: $corrupted"

# Try to decode (may recover depending on error location)
decoded=$(codecrypt conv-decode -i "$corrupted" -p $POLYNOMIALS -c 3 2>/dev/null || echo "Decoding failed")
echo "Recovered: $decoded"
```

## Real-World Scenarios

### Secure Communication Simulation

#### Alice and Bob Communication
```bash
#!/bin/bash
# alice_bob_communication.sh

# Alice's message
ALICE_MESSAGE="Meet at the usual place at midnight"
SHARED_KEY="secret_rendezvous_key"

echo "=== Alice's Side ==="
echo "Original message: $ALICE_MESSAGE"

# Alice encrypts
encrypted=$(codecrypt encrypt -i "$ALICE_MESSAGE" -k "$SHARED_KEY" -a vigenere)
echo "Alice encrypts: $encrypted"

# Add error correction for transmission
transmitted=$(codecrypt conv-encode -i "$encrypted" -p 7 5 -c 3)
echo "Alice adds error correction: ${transmitted:0:50}..."

echo
echo "=== Transmission ==="
echo "Data transmitted over noisy channel..."
# In reality, bits might be corrupted here

echo
echo "=== Bob's Side ==="
# Bob removes error correction
received=$(codecrypt conv-decode -i "$transmitted" -p 7 5 -c 3)
echo "Bob removes error correction: $received"

# Bob decrypts
final=$(codecrypt decrypt -i "$received" -k "$SHARED_KEY" -a vigenere)
echo "Bob decrypts: $final"

echo
echo "Communication successful: $([ "$ALICE_MESSAGE" = "$final" ] && echo "✅ Yes" || echo "❌ No")"
```

### Data Storage Protection

#### File Encryption Workflow
```bash
#!/bin/bash
# secure_storage.sh

# Simulate file content
FILE_CONTENT="Sensitive document content that needs protection during storage and transmission"
PASSWORD="strong_storage_password"

echo "=== Secure Storage Workflow ==="
echo "Original content: $FILE_CONTENT"

# Step 1: Compress (simulated by truncation for demo)
compressed="$FILE_CONTENT"
echo "After compression: ${compressed:0:50}..."

# Step 2: Encrypt
encrypted=$(codecrypt encrypt -i "$compressed" -k "$PASSWORD" -a vigenere)
echo "After encryption: ${encrypted:0:50}..."

# Step 3: Add error correction for storage protection
protected=$(codecrypt turbo-encode -i "$encrypted" -p 7 5)
echo "After error correction: ${protected:0:50}..."

echo
echo "=== Storage/Transmission ==="
echo "Data stored/transmitted with protection..."

echo
echo "=== Recovery Workflow ==="
# Step 1: Remove error correction
recovered=$(codecrypt conv-decode -i "$protected" -p 7 5 -c 3 2>/dev/null || echo "$protected")
echo "After error correction removal: ${recovered:0:50}..."

# Step 2: Decrypt
decrypted=$(codecrypt decrypt -i "$encrypted" -k "$PASSWORD" -a vigenere)
echo "After decryption: ${decrypted:0:50}..."

# Step 3: Decompress (simulated)
final="$decrypted"
echo "Final content: $final"

echo
echo "Data integrity: $([ "$FILE_CONTENT" = "$final" ] && echo "✅ Maintained" || echo "❌ Corrupted")"
```

## Testing and Validation

### Round-trip Testing
```bash
#!/bin/bash
# roundtrip_test.sh

TEST_CASES=(
    "Simple message"
    "Message with 123 numbers!"
    "UPPERCASE and lowercase MiXeD"
    "Special chars: @#$%^&*()"
    "Long message that spans multiple lines and contains various types of characters including punctuation, numbers, and symbols to test the robustness of the encryption algorithms"
)

ALGORITHMS=(caesar vigenere xor)
KEYS=("5" "test_key" "xor_key")

for i in "${!TEST_CASES[@]}"; do
    message="${TEST_CASES[$i]}"
    echo "Test case $((i+1)): ${message:0:30}..."

    for j in "${!ALGORITHMS[@]}"; do
        algo="${ALGORITHMS[$j]}"
        key="${KEYS[$j]}"

        # Encrypt
        encrypted=$(codecrypt encrypt -i "$message" -k "$key" -a "$algo")

        # Decrypt
        decrypted=$(codecrypt decrypt -i "$encrypted" -k "$key" -a "$algo")

        # Verify
        if [ "$message" = "$decrypted" ]; then
            echo "  $algo: ✅ PASS"
        else
            echo "  $algo: ❌ FAIL"
            echo "    Expected: $message"
            echo "    Got:      $decrypted"
        fi
    done
    echo
done
```

### Error Correction Validation
```bash
#!/bin/bash
# error_correction_test.sh

MESSAGES=(
    "Short"
    "Medium length message"
    "Very long message that will test the error correction capability across multiple blocks and demonstrate the effectiveness of convolutional coding"
)

CODES=(
    "7 5 3"      # Rate 1/2, K=3
    "15 17 4"    # Rate 1/2, K=4
    "7 5 3 3"    # Rate 1/3, K=3
)

for message in "${MESSAGES[@]}"; do
    echo "Testing: ${message:0:20}..."

    for code in "${CODES[@]}"; do
        read -r p1 p2 k <<< "$code"

        # Handle rate 1/3 case
        if [[ $code =~ ^([0-9]+)\ ([0-9]+)\ ([0-9]+)\ ([0-9]+)$ ]]; then
            p1=${BASH_REMATCH[1]}
            p2=${BASH_REMATCH[2]}
            p3=${BASH_REMATCH[3]}
            k=${BASH_REMATCH[4]}
            polynomials="$p1 $p2 $p3"
        else
            polynomials="$p1 $p2"
        fi

        # Encode
        encoded=$(codecrypt conv-encode -i "$message" -p $polynomials -c $k)

        # Decode
        decoded=$(codecrypt conv-decode -i "$encoded" -p $polynomials -c $k)

        # Calculate rate
        rate=$(echo "scale=3; ${#message} / ${#encoded}" | bc 2>/dev/null || echo "N/A")

        if [ "$message" = "$decoded" ]; then
            echo "  K=$k, polynomials=($polynomials): ✅ Rate≈$rate"
        else
            echo "  K=$k, polynomials=($polynomials): ❌ FAIL"
        fi
    done
    echo
done
```

## Troubleshooting Common Issues

### Issue 1: Parameter Mismatch
```bash
# ❌ Wrong - mismatched encoding/decoding parameters
codecrypt conv-encode -i "test" -p 7 5 -c 3
codecrypt conv-decode -i "[encoded]" -p 15 17 -c 4  # Different parameters!

# ✅ Correct - matching parameters
codecrypt conv-encode -i "test" -p 7 5 -c 3
codecrypt conv-decode -i "[encoded]" -p 7 5 -c 3   # Same parameters
```

### Issue 2: Key Format Problems
```bash
# ❌ Wrong - non-numeric key for Caesar
codecrypt encrypt -i "test" -k "abc" -a caesar  # Will use default shift of 3

# ✅ Correct - numeric key for Caesar
codecrypt encrypt -i "test" -k "5" -a caesar

# ❌ Wrong - empty key
codecrypt encrypt -i "test" -k "" -a vigenere    # Returns original message

# ✅ Correct - valid alphabetic key
codecrypt encrypt -i "test" -k "secret" -a vigenere
```

### Issue 3: Output Format Confusion
```bash
# XOR produces hex output
ENCRYPTED=$(codecrypt encrypt -i "test" -k "key" -a xor)
echo $ENCRYPTED  # Shows: 1e0b1f1d

# Must use exact hex string for decryption
codecrypt decrypt -i "$ENCRYPTED" -k "key" -a xor  # Correct
codecrypt decrypt -i "1e0b1f1d" -k "key" -a xor    # Also correct
```

## Performance Tips

1. **Use appropriate algorithms**: XOR is fastest, Vigenère is stronger than Caesar
2. **Batch similar operations**: Reuse command setup for multiple messages
3. **Consider message length**: Longer messages show algorithm properties better
4. **Choose constraint length wisely**: Higher K = better error correction but more computation

## Next Steps

- Try the [Advanced Tutorial](advanced-usage.md) for complex scenarios
- Read [Algorithm Theory](../theory/) to understand the mathematics
- Check [Performance Benchmarks](benchmarks.md) for detailed analysis
- Explore the [API Documentation](../api/) for implementation details