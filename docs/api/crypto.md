# Crypto Module API Documentation

The crypto module provides implementations of classical cryptographic algorithms including substitution and stream ciphers.

## Module Structure

```rust
pub mod cipher;
```

## Cipher

### Overview

The `Cipher` struct provides a unified interface for multiple cryptographic algorithms with configurable keys and automatic algorithm dispatch.

### Declaration

```rust
pub struct Cipher {
    algorithm: String,
    key: String,
}
```

### Supported Algorithms

| Algorithm | Type | Key Format | Description |
|-----------|------|------------|-------------|
| `caesar` | Substitution | Integer (0-25) | Fixed shift cipher |
| `vigenere` | Polyalphabetic | Alphabetic string | Keyword-based substitution |
| `xor` | Stream | Any string | XOR with repeating key |

### Methods

#### `new(algorithm: String, key: String) -> Self`

Creates a new cipher instance with specified algorithm and key.

**Parameters:**
- `algorithm`: Algorithm name ("caesar", "vigenere", "xor")
- `key`: Encryption/decryption key (format depends on algorithm)

**Returns:** New `Cipher` instance

**Example:**
```rust
let caesar = Cipher::new("caesar".to_string(), "3".to_string());
let vigenere = Cipher::new("vigenere".to_string(), "secret".to_string());
let xor = Cipher::new("xor".to_string(), "mykey".to_string());
```

#### `encrypt(&self, message: &str) -> String`

Encrypts a message using the configured algorithm and key.

**Parameters:**
- `message`: Plaintext message to encrypt

**Returns:** Encrypted message (format depends on algorithm)

**Example:**
```rust
let cipher = Cipher::new("caesar".to_string(), "3".to_string());
let encrypted = cipher.encrypt("Hello, World!");
// Returns: "Khoor, Zruog!"
```

#### `decrypt(&self, message: &str) -> String`

Decrypts a message using the configured algorithm and key.

**Parameters:**
- `message`: Encrypted message to decrypt

**Returns:** Decrypted plaintext message

**Example:**
```rust
let cipher = Cipher::new("caesar".to_string(), "3".to_string());
let decrypted = cipher.decrypt("Khoor, Zruog!");
// Returns: "Hello, World!"
```

### Algorithm Implementations

## Caesar Cipher

### Overview

Classic substitution cipher that shifts each letter by a fixed number of positions in the alphabet.

### Method Signature

```rust
fn caesar_cipher(&self, message: &str, encrypt: bool) -> String
```

### Algorithm Details

**Encryption Formula:**
```
encrypted_char = (char - base + shift) mod 26 + base
```

**Decryption Formula:**
```
decrypted_char = (char - base + 26 - shift) mod 26 + base
```

Where `base` is 'A' for uppercase and 'a' for lowercase letters.

### Implementation

```rust
fn caesar_cipher(&self, message: &str, encrypt: bool) -> String {
    // Parse the key as a number
    let shift = self.key.parse::<i32>().unwrap_or(3) % 26;
    let shift = if encrypt { shift } else { 26 - shift } as u8;

    message.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let offset = ((c as u8 - base + shift) % 26) + base;
                offset as char
            } else {
                c // Non-alphabetic characters unchanged
            }
        })
        .collect()
}
```

### Features

- **Case Preservation**: Maintains original case of letters
- **Non-alphabetic Handling**: Preserves punctuation, numbers, spaces
- **Modular Arithmetic**: Wraps around alphabet (Z+1 → A)
- **Key Validation**: Invalid keys default to shift of 3

### Usage Examples

```rust
let cipher = Cipher::new("caesar".to_string(), "13".to_string()); // ROT13

// Basic encryption
let encrypted = cipher.encrypt("Hello World");
assert_eq!(encrypted, "Uryyb Jbeyq");

// Case preservation
let encrypted = cipher.encrypt("Hello, World!");
assert_eq!(encrypted, "Uryyb, Jbeyq!");

// Round-trip
let decrypted = cipher.decrypt(&encrypted);
assert_eq!(decrypted, "Hello, World!");
```

### Common Caesar Variants

```rust
// ROT13 (shift 13)
let rot13 = Cipher::new("caesar".to_string(), "13".to_string());

// ROT47 equivalent (not supported - use XOR instead)
// Caesar only supports alphabetic characters

// Custom shifts
let shift_5 = Cipher::new("caesar".to_string(), "5".to_string());
let shift_25 = Cipher::new("caesar".to_string(), "25".to_string()); // Equivalent to shift -1
```

## Vigenère Cipher

### Overview

Polyalphabetic substitution cipher that uses a repeating keyword to vary the shift for each character.

### Method Signature

```rust
fn vigenere_cipher(&self, message: &str, encrypt: bool) -> String
```

### Algorithm Details

**Encryption Formula:**
```
encrypted_char = (message_char + key_char) mod 26
```

**Decryption Formula:**
```
decrypted_char = (message_char - key_char + 26) mod 26
```

### Implementation

```rust
fn vigenere_cipher(&self, message: &str, encrypt: bool) -> String {
    let key = self.key.to_lowercase();
    let key_bytes: Vec<u8> = key.bytes().filter(|&b| b.is_ascii_alphabetic()).collect();

    if key_bytes.is_empty() {
        return message.to_string();
    }

    let mut result = String::with_capacity(message.len());
    let mut key_idx = 0;

    for c in message.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let k = key_bytes[key_idx % key_bytes.len()] - b'a';

            let offset = if encrypt {
                ((c as u8 - base + k) % 26) + base
            } else {
                ((c as u8 - base + 26 - k) % 26) + base
            };

            result.push(offset as char);
            key_idx += 1;
        } else {
            result.push(c);
        }
    }
    result
}
```

### Features

- **Keyword Repetition**: Key repeats to match message length
- **Case Insensitive Key**: Key automatically converted to lowercase
- **Case Preservation**: Maintains original case of message
- **Non-alphabetic Skipping**: Key position only advances for letters
- **Key Filtering**: Only alphabetic characters in key are used

### Usage Examples

```rust
let cipher = Cipher::new("vigenere".to_string(), "SECRET".to_string());

// Basic encryption
let encrypted = cipher.encrypt("HELLO WORLD");
// Key repeats: SECRE TSECR
// Result: Encrypted text

// Case handling
let encrypted = cipher.encrypt("Hello, World!");
let decrypted = cipher.decrypt(&encrypted);
assert_eq!(decrypted, "Hello, World!");

// Key with non-alphabetic characters
let cipher = Cipher::new("vigenere".to_string(), "key123".to_string());
// Effective key: "key" (numbers ignored)
```

### Key Management

```rust
// Short key (repeats)
let cipher1 = Cipher::new("vigenere".to_string(), "key".to_string());
cipher1.encrypt("hello world"); // Key: "keyke ykeyke"

// Long key (truncated to message length)
let cipher2 = Cipher::new("vigenere".to_string(), "verylongkey".to_string());
cipher2.encrypt("hi"); // Uses only "ve"

// Empty key handling
let cipher3 = Cipher::new("vigenere".to_string(), "".to_string());
cipher3.encrypt("test"); // Returns original message unchanged
```

### Security Considerations

- **Key Length**: Longer keys provide better security
- **Key Repetition**: Repeated patterns can be exploited
- **Frequency Analysis**: Vulnerable to statistical attacks
- **Known Plaintext**: Susceptible if plaintext structure is known

## XOR Cipher

### Overview

Stream cipher that applies XOR operation between message bytes and a repeating key.

### Method Signature

```rust
fn xor_cipher(&self, message: &str) -> String
```

### Algorithm Details

**Operation (Encryption and Decryption are identical):**
```
output_byte = input_byte XOR key_byte
```

### Implementation

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

### Features

- **Self-Inverse**: Encryption and decryption are the same operation
- **Binary Safe**: Works with any byte values, not just text
- **Key Repetition**: Key cycles through message
- **Hex Output**: Results encoded as hexadecimal string
- **Variable Key Length**: Accepts keys of any length

### Usage Examples

```rust
let cipher = Cipher::new("xor".to_string(), "secret".to_string());

// Basic operation
let encrypted = cipher.encrypt("Hello, World!");
let decrypted = cipher.decrypt(&encrypted);
assert_eq!(decrypted, "Hello, World!");

// Self-inverse property
let cipher = Cipher::new("xor".to_string(), "key".to_string());
let text = "test";
let encrypted = cipher.encrypt(text);
let decrypted = cipher.decrypt(&encrypted);
assert_eq!(decrypted, text);
```

### Output Format

XOR cipher outputs hexadecimal-encoded strings:

```rust
let cipher = Cipher::new("xor".to_string(), "A".to_string()); // Key: 0x41
let result = cipher.encrypt("B"); // Input: 0x42
// XOR: 0x42 ^ 0x41 = 0x03
// Output: "03"
```

### Key Considerations

```rust
// Binary key (any bytes)
let cipher1 = Cipher::new("xor".to_string(), "\x00\xFF\x55".to_string());

// Text key (converted to bytes)
let cipher2 = Cipher::new("xor".to_string(), "password".to_string());

// Empty key handling
let cipher3 = Cipher::new("xor".to_string(), "".to_string());
// Returns original message unchanged
```

### Security Analysis

- **One-Time Pad**: Secure if key is truly random and used only once
- **Key Reuse**: Vulnerable to cryptanalysis with repeated keys
- **Pattern Recognition**: Short keys create detectable patterns
- **Known Plaintext**: Easy to recover key with known input/output pairs

## Error Handling

### Invalid Algorithm

```rust
let cipher = Cipher::new("invalid".to_string(), "key".to_string());
// This will panic at runtime when encrypt/decrypt is called
```

**Current Implementation:**
```rust
match self.algorithm.as_str() {
    "caesar" => self.caesar_cipher(message, true),
    "vigenere" => self.vigenere_cipher(message, true),
    "xor" => self.xor_cipher(message),
    _ => panic!("Unsupported algorithm: {}", self.algorithm),
}
```

### Recommendations for Production

```rust
// Better error handling (not currently implemented)
pub enum CipherError {
    UnsupportedAlgorithm(String),
    InvalidKey(String),
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, CipherError>;

impl Cipher {
    pub fn encrypt(&self, message: &str) -> Result<String> {
        match self.algorithm.as_str() {
            "caesar" => Ok(self.caesar_cipher(message, true)),
            "vigenere" => Ok(self.vigenere_cipher(message, true)),
            "xor" => Ok(self.xor_cipher(message)),
            _ => Err(CipherError::UnsupportedAlgorithm(self.algorithm.clone())),
        }
    }
}
```

## Testing

The module includes comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_cipher() {
        let cipher = Cipher::new("caesar".to_string(), "3".to_string());
        let message = "Hello, World!";

        let encrypted = cipher.encrypt(message);
        assert_eq!(encrypted, "Khoor, Zruog!");

        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_vigenere_cipher() {
        let cipher = Cipher::new("vigenere".to_string(), "key".to_string());
        let message = "Hello, World!";

        let encrypted = cipher.encrypt(message);
        assert_eq!(encrypted, "Rijvs, Uyvjn!");

        let decrypted = cipher.decrypt(&encrypted);
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_xor_cipher() {
        let cipher = Cipher::new("xor".to_string(), "key".to_string());
        let message = "Hello, World!";

        let encrypted = cipher.encrypt(message);
        let decrypted = cipher.decrypt(&encrypted);

        assert_eq!(decrypted, message);
    }
}
```

### Test Coverage

- **Round-trip Tests**: Encrypt then decrypt returns original
- **Known Vectors**: Specific input/output pairs for verification
- **Edge Cases**: Empty strings, special characters, case sensitivity
- **Key Variations**: Different key formats and lengths

## Performance Characteristics

### Time Complexity

| Algorithm | Encryption | Decryption | Notes |
|-----------|------------|------------|-------|
| Caesar | O(n) | O(n) | Simple character mapping |
| Vigenère | O(n) | O(n) | Key lookup per character |
| XOR | O(n) | O(n) | Bitwise operation per byte |

### Space Complexity

| Algorithm | Memory Usage | Notes |
|-----------|--------------|-------|
| Caesar | O(n) | Output string allocation |
| Vigenère | O(n + k) | Output + key filtering |
| XOR | O(2n) | Hex encoding doubles size |

### Performance Tips

1. **Batch Processing**: Process large texts in chunks
2. **Key Preprocessing**: Pre-filter Vigenère keys
3. **Memory Reuse**: Reuse cipher instances for multiple operations
4. **Algorithm Selection**: Choose based on security vs. performance needs

## Integration Examples

### CLI Integration

```rust
use crate::crypto::cipher::Cipher;

// From CLI arguments
let cipher = Cipher::new(algorithm, key);
match operation {
    "encrypt" => println!("{}", cipher.encrypt(&input)),
    "decrypt" => println!("{}", cipher.decrypt(&input)),
}
```

### Batch Processing

```rust
let cipher = Cipher::new("vigenere".to_string(), "secret".to_string());
let messages = vec!["msg1", "msg2", "msg3"];

let encrypted: Vec<String> = messages
    .iter()
    .map(|msg| cipher.encrypt(msg))
    .collect();
```

### Algorithm Comparison

```rust
let message = "Test message for comparison";
let key = "testkey";

let algorithms = vec!["caesar", "vigenere", "xor"];
for algo in algorithms {
    let cipher = Cipher::new(algo.to_string(), key.to_string());
    let encrypted = cipher.encrypt(message);
    println!("{}: {}", algo, encrypted);
}
```

## See Also

- [Cryptography Theory](../theory/cryptography.md)
- [Crypto Tutorial](../tutorials/crypto-tutorial.md)
- [Security Analysis](../theory/cryptography.md#security-analysis)
- [Performance Benchmarks](../examples/benchmarks.md#cryptography)