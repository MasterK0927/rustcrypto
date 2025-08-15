# Cryptography Theory

This document covers the theoretical foundations of the cryptographic algorithms implemented in CodeCrypt, including classical ciphers and their security properties.

## Table of Contents

- [Introduction to Cryptography](#introduction-to-cryptography)
- [Classical Cryptography](#classical-cryptography)
- [Substitution Ciphers](#substitution-ciphers)
- [Stream Ciphers](#stream-ciphers)
- [Cryptanalysis Methods](#cryptanalysis-methods)
- [Security Analysis](#security-analysis)
- [Modern Context](#modern-context)

## Introduction to Cryptography

### Fundamental Concepts

**Cryptography** is the science of secure communication in the presence of adversaries. It encompasses:

- **Encryption**: Transforming plaintext into ciphertext
- **Decryption**: Recovering plaintext from ciphertext
- **Key Management**: Generation, distribution, and storage of keys
- **Authentication**: Verifying identity and data integrity

### Basic Terminology

| Term | Definition |
|------|------------|
| **Plaintext** | Original readable message |
| **Ciphertext** | Encrypted message |
| **Key** | Secret parameter used in encryption/decryption |
| **Algorithm/Cipher** | Mathematical procedure for encryption/decryption |
| **Cryptanalysis** | Science of breaking cryptographic systems |
| **Cryptology** | Combined study of cryptography and cryptanalysis |

### Security Models

**Kerckhoffs' Principles (1883):**
1. System should be practically unbreakable
2. Compromise of system should not inconvenience correspondents
3. Key should be memorable without notes
4. Ciphertext should be transmittable by telegraph
5. Apparatus should be portable and usable by single person
6. **System should be easy to use**

**Modern Interpretation:**
- Security should rely on key secrecy, not algorithm secrecy
- Algorithms should be publicly scrutinized
- Open design leads to better security

### Attack Models

**Based on Adversary's Knowledge:**
1. **Ciphertext-only**: Only ciphertext available
2. **Known-plaintext**: Some plaintext-ciphertext pairs known
3. **Chosen-plaintext**: Adversary can choose plaintexts to encrypt
4. **Chosen-ciphertext**: Adversary can choose ciphertexts to decrypt
5. **Adaptive chosen-text**: Interactive chosen-plaintext/ciphertext attacks

## Classical Cryptography

### Historical Context

Classical cryptography primarily deals with **linguistic cryptography** - ciphers that operate on natural language text and can often be broken using statistical properties of language.

**Characteristics:**
- Manual implementation (pre-computer era)
- Character-based operations
- Vulnerable to frequency analysis
- Educational and historical value

### Classification

**By Operation Type:**
1. **Substitution**: Replace characters with other characters
2. **Transposition**: Rearrange character positions
3. **Product**: Combination of substitution and transposition

**By Key Usage:**
1. **Symmetric**: Same key for encryption and decryption
2. **Asymmetric**: Different keys for encryption and decryption (not in classical era)

## Substitution Ciphers

### Monoalphabetic Substitution

**Definition**: Each plaintext character is replaced by exactly one ciphertext character throughout the message.

#### Caesar Cipher

**Mathematical Formulation:**
```
Encryption: C ≡ (P + k) mod 26
Decryption: P ≡ (C - k) mod 26
```
where P is plaintext character, C is ciphertext character, k is shift value.

**Algorithm Analysis:**
- **Key Space**: 26 possible keys (including identity)
- **Practical Keys**: 25 (excluding no shift)
- **Security**: Extremely weak, vulnerable to brute force

**Implementation Details:**
```rust
// Character mapping for uppercase
let base = b'A';
let encrypted_char = ((char as u8 - base + shift) % 26) + base;

// Case preservation
let base = if char.is_ascii_lowercase() { b'a' } else { b'A' };
```

**Cryptanalysis:**
1. **Brute Force**: Try all 25 possible shifts
2. **Frequency Analysis**: Most common letter likely maps to 'E'
3. **Pattern Recognition**: Look for common words like "THE"

**Historical Significance:**
- Used by Julius Caesar (shift of 3)
- ROT13 (shift of 13) used in modern contexts for text obfuscation
- Foundational cipher for understanding cryptographic concepts

#### General Monoalphabetic Substitution

**Key Space**: 26! ≈ 4 × 10²⁶ possible keys

**Despite large key space, still vulnerable to frequency analysis:**

**English Letter Frequencies:**
```
E: 12.7%    T: 9.1%     A: 8.1%     O: 7.5%
I: 7.0%     N: 6.7%     S: 6.3%     H: 6.1%
R: 6.0%     D: 4.3%     L: 4.0%     C: 2.8%
```

**Cryptanalysis Approach:**
1. Count frequency of each ciphertext character
2. Match high-frequency characters to common English letters
3. Use digram and trigram analysis for refinement
4. Apply linguistic constraints and trial-and-error

### Polyalphabetic Substitution

**Definition**: Use multiple substitution alphabets, changing the substitution rule during encryption.

#### Vigenère Cipher

**Mathematical Formulation:**
```
Encryption: C_i ≡ (P_i + K_{i mod m}) mod 26
Decryption: P_i ≡ (C_i - K_{i mod m}) mod 26
```
where m is key length.

**Algorithm Analysis:**
- **Key**: Repeating keyword
- **Security**: Stronger than monoalphabetic, but still breakable
- **Weakness**: Key repetition creates patterns

**Example:**
```
Plaintext:  ATTACKATDAWN
Key:        LEMONLEMONLE
Ciphertext: LXFOPVEFRNHR
```

**Implementation Considerations:**
```rust
// Key character cycling
let key_char = key_bytes[key_index % key_bytes.len()];

// Only advance key for alphabetic characters
if char.is_ascii_alphabetic() {
    key_index += 1;
}
```

**Cryptanalysis:**

1. **Kasiski Examination**: Find repeated patterns in ciphertext
   - Repeated patterns suggest same plaintext encrypted with same key portion
   - Distance between repetitions likely multiple of key length

2. **Index of Coincidence**: Measure of character distribution
   ```
   IC = Σ(i=0 to 25) [n_i(n_i-1)] / [N(N-1)]
   ```
   - Random text: IC ≈ 0.038
   - English text: IC ≈ 0.067
   - Use to estimate key length

3. **Frequency Analysis per Key Position**: Once key length determined
   - Separate ciphertext into subsequences by key position
   - Each subsequence is Caesar cipher
   - Apply frequency analysis to each subsequence

**Security Improvements:**
- **One-Time Pad**: Key as long as message, truly random, never reused
- **Running Key**: Use text from book as key (still vulnerable)

## Stream Ciphers

### XOR Cipher

**Mathematical Foundation:**
```
Encryption: C_i = P_i ⊕ K_i
Decryption: P_i = C_i ⊕ K_i
```

**XOR Properties:**
- **Commutative**: a ⊕ b = b ⊕ a
- **Associative**: (a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)
- **Self-inverse**: a ⊕ a = 0
- **Identity**: a ⊕ 0 = a

**Key Insights:**
1. Encryption and decryption are identical operations
2. Perfect secrecy if key is truly random and used only once
3. Vulnerable if key is reused or predictable

**Implementation:**
```rust
// Byte-wise XOR with repeating key
let result_byte = message_byte ^ key_bytes[index % key_length];

// Hex encoding for output
format!("{:02x}", result_byte)
```

**Security Analysis:**

**One-Time Pad (Theoretical Optimum):**
- Key requirements:
  - Length ≥ message length
  - Truly random
  - Never reused
  - Kept secret
- **Shannon's Theorem**: One-time pad provides perfect secrecy

**Practical XOR Cipher (Weak):**
- Repeating key creates vulnerabilities
- Known plaintext reveals key portions
- Crib-dragging attacks on reused keys

**Common Attacks:**
1. **Key Recovery**: If plaintext known at any position
2. **Ciphertext-Only**: Statistical analysis of repeated key patterns
3. **Many-Time Pad**: XOR multiple ciphertexts encrypted with same key

## Cryptanalysis Methods

### Frequency Analysis

**Single Character Frequencies:**
- Count occurrence of each character in ciphertext
- Compare with expected frequencies in target language
- Most effective against monoalphabetic substitution

**Bigram and Trigram Analysis:**
- Examine pairs and triplets of characters
- Common English bigrams: TH, HE, IN, ER, AN
- Common trigrams: THE, AND, ING, HER, HAT

**Statistical Tests:**
- **Chi-squared test**: Measure deviation from expected frequencies
- **Index of Coincidence**: Measure of non-randomness
- **Mutual Index of Coincidence**: Compare frequency distributions

### Pattern Recognition

**Repeated Patterns:**
- Look for repeated sequences in ciphertext
- May indicate repeated plaintext with same key portion
- Kasiski examination for polyalphabetic ciphers

**Word Patterns:**
- Identify likely positions of common words
- Use known word structures (articles, conjunctions)
- Dictionary attacks with partial decryption

### Mathematical Approaches

**Linear Algebra:**
- Hill cipher: Matrix operations in mod 26
- System of linear equations for key recovery

**Number Theory:**
- Modular arithmetic for shift-based ciphers
- Greatest common divisor for key period finding

**Information Theory:**
- Entropy measurements
- Redundancy exploitation
- Compression-based analysis

## Security Analysis

### Security Metrics

**Computational Security:**
- Security based on computational difficulty
- Measured in time/resources required to break
- Practical security for reasonable time horizons

**Information-Theoretic Security:**
- Security regardless of computational power
- Perfect secrecy: ciphertext provides no information about plaintext
- Only achieved by one-time pad

**Provable Security:**
- Security reduced to known hard problems
- Formal security definitions and proofs
- Not applicable to classical ciphers

### Vulnerability Assessment

#### Caesar Cipher
- **Key Space**: 25 keys → brute force trivial
- **Frequency Preservation**: Character frequencies unchanged
- **Pattern Preservation**: Word patterns visible
- **Security**: None against any serious attack

#### Vigenère Cipher
- **Key Space**: 26^m keys (m = key length)
- **Frequency Dilution**: Spreads frequencies across multiple alphabets
- **Pattern Disruption**: Breaks simple frequency analysis
- **Weakness**: Key repetition creates patterns
- **Security**: Moderate against casual attacks, weak against systematic cryptanalysis

#### XOR Cipher
- **Perfect Security**: If key is one-time pad
- **Practical Security**: Very weak with repeating keys
- **Key Reuse**: Catastrophic security failure
- **Known Plaintext**: Complete key recovery possible

### Attack Complexity

**Time Complexity:**
- Caesar: O(1) - try all 25 keys
- Vigenère: O(m × 26^m) for key length m
- XOR (repeating): O(n × k) where n is message length, k is key length

**Space Complexity:**
- Generally O(1) for manual methods
- O(alphabet_size) for frequency tables
- O(key_space) for exhaustive search storage

**Success Probability:**
- Depends on message length, key quality, and attack method
- Longer messages provide more statistical information
- Shorter keys are easier to determine

## Modern Context

### Educational Value

**Learning Cryptographic Concepts:**
- Basic encryption/decryption operations
- Key management principles
- Attack methodologies
- Security trade-offs

**Programming Implementation:**
- Character encoding (ASCII, UTF-8)
- Modular arithmetic
- Bit manipulation (XOR)
- String processing

### Practical Applications

**Modern Uses of Classical Ciphers:**
- **ROT13**: Text obfuscation (not security)
- **XOR**: Component in stream ciphers and OTP
- **Teaching Tool**: Cryptography education
- **Puzzles**: Recreational cryptography

**Legacy Systems:**
- Understanding old encrypted documents
- Historical document analysis
- Archaeological cryptographic artifacts

### Limitations in Modern Context

**Why Classical Ciphers Are Inadequate:**
1. **Computational Power**: Modern computers break them instantly
2. **Statistical Analysis**: Automated frequency analysis
3. **Linguistic Patterns**: Natural language provides too much structure
4. **Key Management**: Manual key distribution impractical
5. **Scale**: Inadequate for high-volume digital communications

### Evolution to Modern Cryptography

**Improvements Needed:**
- **Confusion**: Obscure relationship between key and ciphertext
- **Diffusion**: Spread plaintext statistics throughout ciphertext
- **Non-linearity**: Resist linear cryptanalysis
- **Large Key Spaces**: Resist brute force attacks

**Modern Successors:**
- **Block Ciphers**: DES, AES
- **Stream Ciphers**: RC4, ChaCha20
- **Public Key**: RSA, ECC
- **Hash Functions**: SHA-256, SHA-3

## Implementation Security

### Common Programming Vulnerabilities

**Buffer Overflows:**
```rust
// Rust prevents buffer overflows at compile time
let mut output = vec![0; input.len()]; // Safe allocation
```

**Integer Overflow:**
```rust
// Use wrapping arithmetic for modular operations
let result = (char as u8).wrapping_add(shift) % 26;
```

**Key Handling:**
```rust
// Clear sensitive data
key.zeroize(); // Not implemented in current version
```

### Side-Channel Attacks

**Timing Attacks:**
- Constant-time implementations
- Avoid conditional operations based on secret data

**Power Analysis:**
- Less relevant for software-only implementations
- Important for embedded systems

**Cache Attacks:**
- Use data-independent memory access patterns
- Avoid lookup tables indexed by secret data

## Conclusion

Classical cryptography provides foundational understanding of cryptographic principles while demonstrating the importance of:

1. **Mathematical Rigor**: Formal analysis of security properties
2. **Attack Methodology**: Systematic approaches to cryptanalysis
3. **Implementation Details**: Proper handling of edge cases and security considerations
4. **Historical Context**: Evolution of cryptographic thinking

While inadequate for modern security needs, these algorithms remain valuable for:
- Education and understanding cryptographic principles
- Historical document analysis
- Recreational cryptography and puzzles
- Building blocks for more complex systems

The implementations in CodeCrypt demonstrate both the algorithms and their limitations, providing a bridge between classical and modern cryptographic understanding.

## See Also

- [Crypto API Documentation](../api/crypto.md)
- [Cryptography Tutorial](../tutorials/crypto-tutorial.md)
- [Security Analysis Examples](../examples/security-analysis.md)
- [Modern Cryptography Context](modern-cryptography.md)