# CLI Module API Documentation

The CLI module provides command-line interface definitions and argument parsing for CodeCrypt using the `clap` crate.

## Module Overview

The CLI module defines the complete command-line interface structure, including all subcommands, their parameters, and validation rules.

## Structures

### Cli

Main CLI structure that serves as the entry point for argument parsing.

```rust
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}
```

**Attributes:**
- Uses `clap::Parser` derive macro for automatic CLI generation
- Inherits author, version, and description from `Cargo.toml`
- Contains a single subcommand field

### Command

Enum defining all available subcommands and their parameters.

```rust
#[derive(Subcommand)]
pub enum Command {
    ConvEncode { /* parameters */ },
    ConvDecode { /* parameters */ },
    TurboEncode { /* parameters */ },
    Encrypt { /* parameters */ },
    Decrypt { /* parameters */ },
}
```

## Commands

### ConvEncode

Convolutional encoding command.

```rust
ConvEncode {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, num_args = 1.., value_parser = clap::value_parser!(u32))]
    polynomials: Vec<u32>,

    #[arg(short, long, default_value_t = 3)]
    constraint_length: u32,
}
```

**Parameters:**
- `input` (`-i`, `--input`): Input message string to encode
- `polynomials` (`-p`, `--polynomials`): Generator polynomials in octal notation (multiple values)
- `constraint_length` (`-c`, `--constraint-length`): Constraint length K (default: 3)

**Usage Examples:**
```bash
codecrypt conv-encode -i "Hello" -p 7 5 -c 3
codecrypt conv-encode --input "Test" --polynomials 15 17 --constraint-length 4
```

**Validation:**
- `polynomials` requires at least one value
- All polynomial values must be valid u32 integers
- `constraint_length` defaults to 3 if not specified

### ConvDecode

Convolutional decoding command using Viterbi algorithm.

```rust
ConvDecode {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, num_args = 1.., value_parser = clap::value_parser!(u32))]
    polynomials: Vec<u32>,

    #[arg(short, long, default_value_t = 3)]
    constraint_length: u32,
}
```

**Parameters:**
- `input` (`-i`, `--input`): Encoded message string to decode
- `polynomials` (`-p`, `--polynomials`): Generator polynomials (must match encoding)
- `constraint_length` (`-c`, `--constraint-length`): Constraint length (must match encoding)

**Usage Examples:**
```bash
codecrypt conv-decode -i "[encoded_data]" -p 7 5 -c 3
codecrypt conv-decode --input "[encoded_data]" --polynomials 7 5 --constraint-length 3
```

**Important Notes:**
- Parameters must exactly match those used for encoding
- Mismatched parameters will result in incorrect decoding
- Input should be the exact output from `conv-encode`

### TurboEncode

Turbo encoding command using parallel concatenated convolutional codes.

```rust
TurboEncode {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, num_args = 1.., value_parser = clap::value_parser!(u32))]
    polynomials: Vec<u32>,
}
```

**Parameters:**
- `input` (`-i`, `--input`): Input message string to turbo encode
- `polynomials` (`-p`, `--polynomials`): Generator polynomials for constituent encoders

**Usage Examples:**
```bash
codecrypt turbo-encode -i "Data" -p 7 5
codecrypt turbo-encode --input "Critical Data" --polynomials 7 5
```

**Algorithm Details:**
- Uses two identical convolutional encoders with constraint length 3
- Applies interleaving between constituent encoders
- Produces systematic output (original + 2× parity bits)

### Encrypt

Message encryption command supporting multiple cipher algorithms.

```rust
Encrypt {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    key: String,

    #[arg(short, long, default_value = "xor")]
    algorithm: String,
}
```

**Parameters:**
- `input` (`-i`, `--input`): Plaintext message to encrypt
- `key` (`-k`, `--key`): Encryption key (format depends on algorithm)
- `algorithm` (`-a`, `--algorithm`): Cipher algorithm to use (default: "xor")

**Supported Algorithms:**
- `caesar`: Caesar cipher with numeric shift key
- `vigenere`: Vigenère cipher with alphabetic keyword
- `xor`: XOR cipher with arbitrary string key

**Usage Examples:**
```bash
# Caesar cipher
codecrypt encrypt -i "Hello World" -k "3" -a caesar

# Vigenère cipher
codecrypt encrypt -i "Secret Message" -k "password" -a vigenere

# XOR cipher (default)
codecrypt encrypt -i "Binary Data" -k "secret"
codecrypt encrypt -i "Binary Data" -k "secret" -a xor
```

**Key Format Requirements:**
- **Caesar**: Numeric string (e.g., "5", "13")
- **Vigenère**: Alphabetic string (e.g., "secret", "keyword")
- **XOR**: Any string (binary safe)

### Decrypt

Message decryption command (inverse of encrypt).

```rust
Decrypt {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    key: String,

    #[arg(short, long, default_value = "xor")]
    algorithm: String,
}
```

**Parameters:**
- `input` (`-i`, `--input`): Ciphertext message to decrypt
- `key` (`-k`, `--key`): Decryption key (must match encryption key)
- `algorithm` (`-a`, `--algorithm`): Cipher algorithm used for encryption

**Usage Examples:**
```bash
# Decrypt Caesar cipher
codecrypt decrypt -i "Khoor Zruog" -k "3" -a caesar

# Decrypt Vigenère cipher
codecrypt decrypt -i "[encrypted_text]" -k "password" -a vigenere

# Decrypt XOR cipher
codecrypt decrypt -i "1a0b3c2d" -k "secret" -a xor
```

**Important Notes:**
- Key and algorithm must exactly match those used for encryption
- XOR cipher input should be hexadecimal string
- Text cipher outputs are human-readable

## Functions

### parse_args()

Entry point function for CLI argument parsing.

```rust
pub fn parse_args() -> Cli {
    Cli::parse()
}
```

**Returns:** Parsed `Cli` structure with validated arguments

**Usage in main.rs:**
```rust
fn main() {
    let cli = cli::parse_args();
    match cli.command {
        // Handle each command variant
    }
}
```

**Error Handling:**
- Automatically generates help text for `--help` flag
- Validates argument types and constraints
- Exits with error message for invalid arguments
- Provides usage examples and parameter descriptions

## CLI Features

### Automatic Help Generation

The CLI automatically generates comprehensive help text:

```bash
# General help
$ codecrypt --help
A CLI tool for convolutional coding and basic cryptography

Usage: codecrypt <COMMAND>

Commands:
  conv-encode   Convolutional encoding
  conv-decode   Convolutional decoding
  turbo-encode  Turbo encoding
  encrypt       Encrypt a message
  decrypt       Decrypt a message
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```bash
# Command-specific help
$ codecrypt conv-encode --help
Convolutional encoding

Usage: codecrypt conv-encode --input <INPUT> --polynomials <POLYNOMIALS>... [OPTIONS]

Options:
  -i, --input <INPUT>                          Input message
  -p, --polynomials <POLYNOMIALS>...           Generator polynomials
  -c, --constraint-length <CONSTRAINT_LENGTH>  Constraint length [default: 3]
  -h, --help                                   Print help
```

### Parameter Validation

#### Type Validation
```rust
// Polynomials must be valid u32 values
#[arg(short, long, num_args = 1.., value_parser = clap::value_parser!(u32))]
polynomials: Vec<u32>,
```

**Example:**
```bash
$ codecrypt conv-encode -i "test" -p abc
error: invalid value 'abc' for '--polynomials <POLYNOMIALS>...': invalid digit found in string
```

#### Range Validation
```rust
// Default values provide sensible defaults
#[arg(short, long, default_value_t = 3)]
constraint_length: u32,
```

#### Multiple Value Handling
```rust
// Accepts multiple polynomial values
#[arg(short, long, num_args = 1.., value_parser = clap::value_parser!(u32))]
polynomials: Vec<u32>,
```

**Example:**
```bash
# Valid: multiple polynomials
$ codecrypt conv-encode -i "test" -p 7 5 3

# Valid: single polynomial
$ codecrypt conv-encode -i "test" -p 7

# Invalid: no polynomials
$ codecrypt conv-encode -i "test" -p
```

### Version Information

Automatically extracts version from `Cargo.toml`:

```bash
$ codecrypt --version
codecrypt 0.1.0
```

## Integration with Main Application

### Command Dispatching

The main application uses pattern matching on the `Command` enum:

```rust
fn main() {
    let cli = cli::parse_args();

    match cli.command {
        Command::ConvEncode { input, polynomials, constraint_length } => {
            let mut encoder = convolutional::encoder::ConvolutionalEncoder::new(
                constraint_length,
                polynomials
            );
            // Process encoding...
        },

        Command::Encrypt { input, key, algorithm } => {
            let cipher = crypto::cipher::Cipher::new(algorithm, key);
            let encrypted = cipher.encrypt(&input);
            println!("Encrypted: {}", encrypted);
        },

        // Other command handlers...
    }
}
```

### Data Conversion Pipeline

The CLI handles string input/output while algorithms work with bit vectors:

```rust
// String to bit conversion (in main.rs)
let input_bits = input.chars()
    .flat_map(|c| {
        let byte = c as u8;
        (0..8).map(move |i| (byte >> (7 - i)) & 1 == 1)
    })
    .collect::<Vec<_>>();

// Algorithm processing
let encoded = encoder.encode(&input_bits);

// Bit to string conversion
let result = encoded.chunks(8)
    .map(|chunk| {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate().take(8) {
            if bit {
                byte |= 1 << (7 - i);
            }
        }
        byte as char
    })
    .collect::<String>();
```

## Error Handling

### Clap Error Handling

Clap automatically handles:
- Invalid argument types
- Missing required parameters
- Unknown flags or commands
- Help text generation

### Application Error Handling

Currently uses panic for algorithm errors:
```rust
match cli.command {
    Command::ConvEncode { input, polynomials, constraint_length } => {
        let mut encoder = convolutional::encoder::ConvolutionalEncoder::new(
            constraint_length,
            polynomials
        );
        // May panic on invalid parameters
        let encoded = encoder.encode(&input_bits);
        println!("Encoded: {}", result);
    },
}
```

**Future Enhancement:**
```rust
// Improved error handling with Results
match cli.command {
    Command::ConvEncode { input, polynomials, constraint_length } => {
        match ConvolutionalEncoder::new(constraint_length, polynomials) {
            Ok(mut encoder) => {
                match encoder.encode(&input_bits) {
                    Ok(encoded) => println!("Encoded: {}", encoded),
                    Err(e) => eprintln!("Encoding error: {}", e),
                }
            },
            Err(e) => eprintln!("Configuration error: {}", e),
        }
    },
}
```

## Extension Guidelines

### Adding New Commands

1. **Add to Command Enum:**
```rust
#[derive(Subcommand)]
pub enum Command {
    // ... existing commands ...

    NewCommand {
        #[arg(short, long)]
        parameter1: String,

        #[arg(short, long, default_value_t = 42)]
        parameter2: u32,
    },
}
```

2. **Update main.rs Handler:**
```rust
match cli.command {
    // ... existing handlers ...

    Command::NewCommand { parameter1, parameter2 } => {
        // Implementation
    },
}
```

3. **Add Documentation:**
- Update this API documentation
- Add usage examples
- Include parameter descriptions

### Parameter Design Guidelines

**Naming Conventions:**
- Use descriptive, unambiguous names
- Follow kebab-case for long options
- Use single letters for common short options

**Default Values:**
- Provide sensible defaults where possible
- Use standard values from literature
- Document default behavior clearly

**Validation:**
- Use appropriate clap validators
- Provide clear error messages
- Document valid ranges and formats

**Example Good Parameter Design:**
```rust
NewAlgorithm {
    #[arg(short, long, help = "Input data to process")]
    input: String,

    #[arg(short, long, default_value_t = 256, help = "Block size in bits")]
    block_size: usize,

    #[arg(short, long, value_parser = parse_hex_key, help = "Encryption key in hex format")]
    key: Vec<u8>,

    #[arg(long, help = "Enable verbose output")]
    verbose: bool,
}
```

## Performance Considerations

### Argument Parsing Overhead
- Clap parsing is typically negligible compared to algorithm execution
- String allocations for parameters are minimal
- No performance impact on algorithm implementations

### Memory Usage
- CLI structures are small and short-lived
- Parameter vectors (e.g., polynomials) are typically small
- No memory leaks from argument parsing

## Testing CLI

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_conv_encode_parsing() {
        let args = vec![
            "codecrypt", "conv-encode",
            "-i", "test",
            "-p", "7", "5",
            "-c", "3"
        ];

        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Command::ConvEncode { input, polynomials, constraint_length } => {
                assert_eq!(input, "test");
                assert_eq!(polynomials, vec![7, 5]);
                assert_eq!(constraint_length, 3);
            },
            _ => panic!("Wrong command parsed"),
        }
    }
}
```

### Integration Testing
```rust
#[test]
fn test_full_command_execution() {
    use std::process::Command;

    let output = Command::new("cargo")
        .args(&["run", "--", "encrypt", "-i", "test", "-k", "key"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Encrypted:"));
}
```

## See Also

- [Main Application](../architecture.md#main-orchestrator) - How CLI integrates with the main application
- [Command Examples](../examples/basic-examples.md) - Practical usage examples
- [Contributing Guide](../development/contributing.md) - Adding new commands
- [Clap Documentation](https://docs.rs/clap/) - External clap crate documentation