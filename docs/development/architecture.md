# Architecture Overview

This document provides a comprehensive overview of the CodeCrypt architecture, design decisions, and system organization.

## Table of Contents

- [System Overview](#system-overview)
- [Module Architecture](#module-architecture)
- [Design Principles](#design-principles)
- [Data Flow](#data-flow)
- [Implementation Patterns](#implementation-patterns)
- [Extension Points](#extension-points)
- [Performance Considerations](#performance-considerations)
- [Future Architecture Evolution](#future-architecture-evolution)

## System Overview

### High-Level Architecture

CodeCrypt is designed as a modular CLI application with clean separation between different cryptographic and error correction domains.

```
┌─────────────────────────────────────────────┐
│                CLI Layer                     │
│  (Argument parsing, command dispatch)       │
└─────────────┬───────────────────────────────┘
              │
┌─────────────▼───────────────────────────────┐
│              Main Orchestrator               │
│     (Command routing and execution)          │
└─────┬───────────────────────┬───────────────┘
      │                       │
┌─────▼─────────┐    ┌────────▼──────────┐
│  Cryptographic │    │ Error Correction  │
│    Modules     │    │     Modules       │
│                │    │                   │
│ ┌─────────────┐│    │ ┌─────────────────┐│
│ │   Cipher    ││    │ │ Convolutional   ││
│ │ Algorithms  ││    │ │   Encoder       ││
│ └─────────────┘│    │ └─────────────────┘│
│                │    │ ┌─────────────────┐│
│                │    │ │    Viterbi      ││
│                │    │ │    Decoder      ││
│                │    │ └─────────────────┘│
│                │    │ ┌─────────────────┐│
│                │    │ │     Turbo       ││
│                │    │ │    Encoder      ││
│                │    │ └─────────────────┘│
└────────────────┘    └───────────────────────┘
```

### Core Components

1. **CLI Interface**: Command-line argument parsing and user interaction
2. **Algorithm Implementations**: Core cryptographic and error correction algorithms
3. **Data Processing Pipeline**: Bit manipulation and format conversion utilities
4. **Testing Framework**: Comprehensive test suite with known answer validation

## Module Architecture

### Directory Structure Analysis

```
src/
├── main.rs              # Entry point and command orchestration
├── cli.rs               # Command-line interface definitions
├── convolutional/       # Convolutional coding implementations
│   ├── mod.rs          # Module exports and common types
│   ├── encoder.rs      # Convolutional encoder implementation
│   └── viterbi.rs      # Viterbi decoder implementation
├── turbo/              # Turbo coding implementations
│   ├── mod.rs          # Module exports
│   └── encoder.rs      # Turbo encoder with interleaving
└── crypto/             # Cryptographic algorithms
    ├── mod.rs          # Crypto module exports
    └── cipher.rs       # Classical cipher implementations
```

### Module Responsibilities

#### CLI Module (`cli.rs`)
**Purpose**: Define command-line interface and argument parsing

**Key Components:**
```rust
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

pub enum Command {
    ConvEncode { /* ... */ },
    ConvDecode { /* ... */ },
    TurboEncode { /* ... */ },
    Encrypt { /* ... */ },
    Decrypt { /* ... */ },
}
```

**Responsibilities:**
- Define command structure using `clap` derive macros
- Validate command-line arguments
- Provide help and usage information
- Type-safe argument parsing

**Design Rationale:**
- Separation of CLI concerns from business logic
- Extensible command structure for new algorithms
- Consistent parameter naming across commands

#### Main Orchestrator (`main.rs`)
**Purpose**: Route commands and orchestrate execution

**Key Functions:**
```rust
fn main() {
    let cli = cli::parse_args();
    match cli.command {
        Command::ConvEncode { input, polynomials, constraint_length } => {
            // Instantiate encoder and process
        },
        Command::Encrypt { input, key, algorithm } => {
            // Instantiate cipher and process
        },
        // ... other commands
    }
}
```

**Responsibilities:**
- Parse CLI arguments
- Instantiate appropriate algorithm implementations
- Handle data format conversions (string ↔ bits)
- Coordinate input/output operations

**Design Decisions:**
- Single-responsibility pattern: main.rs only coordinates
- No business logic in main - delegates to modules
- Consistent error handling across all commands

### Algorithm Module Architecture

#### Convolutional Module
**Structure:**
```rust
// mod.rs - Public API
pub mod encoder;
pub mod viterbi;

// encoder.rs
pub struct ConvolutionalEncoder {
    constraint_length: u32,
    polynomials: Vec<u32>,
    rate_denominator: usize,
    state: u32,
}

// viterbi.rs
pub struct ViterbiDecoder {
    constraint_length: u32,
    polynomials: Vec<u32>,
    num_states: usize,
    rate_denominator: usize,
}
```

**Key Design Patterns:**
1. **State Management**: Encoders maintain internal state between operations
2. **Polynomial Representation**: Automatic octal-to-binary conversion
3. **Generic Rate Support**: Configurable rate through polynomial count
4. **Separation of Concerns**: Encoder and decoder are independent implementations

#### Crypto Module
**Structure:**
```rust
// cipher.rs
pub struct Cipher {
    algorithm: String,
    key: String,
}

impl Cipher {
    // Algorithm dispatch through method calls
    fn caesar_cipher(&self, message: &str, encrypt: bool) -> String;
    fn vigenere_cipher(&self, message: &str, encrypt: bool) -> String;
    fn xor_cipher(&self, message: &str) -> String;
}
```

**Key Design Patterns:**
1. **Strategy Pattern**: Algorithm selection through string parameter
2. **Unified Interface**: Single encrypt/decrypt methods for all algorithms
3. **Self-Contained**: Each algorithm implementation is independent
4. **Stateless Operations**: No state maintained between operations

## Design Principles

### 1. Educational Focus
**Principle**: Code should be clear and educational rather than optimally efficient

**Implementation:**
- Verbose variable names that match mathematical notation
- Extensive comments explaining algorithm steps
- Direct implementation of textbook algorithms
- Preference for clarity over micro-optimizations

**Example:**
```rust
// Clear mathematical operations with explanatory comments
let mut parity = 0;
let mut temp = self.state & poly;

// Calculate XOR parity of polynomial taps
while temp != 0 {
    parity ^= temp & 1;  // XOR each bit
    temp >>= 1;          // Shift to next bit
}
```

### 2. Type Safety
**Principle**: Use Rust's type system to prevent errors

**Implementation:**
- Strong typing for different data representations
- Builder patterns for complex configurations
- Option/Result types for error handling
- Const generics where appropriate

**Example:**
```rust
// Type-safe polynomial representation
pub fn new(constraint_length: u32, polynomials: Vec<u32>) -> Self {
    // Validate constraint length at construction time
    assert!(constraint_length >= 2 && constraint_length <= 16);
    // ...
}
```

### 3. Modularity
**Principle**: Each algorithm should be independently usable

**Implementation:**
- No cross-dependencies between algorithm modules
- Self-contained implementations
- Clear module boundaries
- Consistent API patterns

### 4. Extensibility
**Principle**: Easy to add new algorithms without modifying existing code

**Implementation:**
- Plugin-style architecture for new ciphers
- Consistent trait patterns (potential future enhancement)
- Modular command structure
- Template patterns for similar algorithms

## Data Flow

### Input Processing Pipeline

```
User Input (String)
        ↓
CLI Argument Parsing (clap)
        ↓
Command Dispatching (main.rs)
        ↓
String → Bit Conversion
        ↓
Algorithm Processing
        ↓
Bit → String Conversion
        ↓
Output Display
```

### Bit Conversion Logic

#### String to Bits
```rust
let input_bits = input.chars()
    .flat_map(|c| {
        let byte = c as u8;
        (0..8).map(move |i| (byte >> (7 - i)) & 1 == 1)
    })
    .collect::<Vec<_>>();
```

**Rationale:**
- MSB-first bit ordering (network byte order)
- Boolean representation for clarity
- Iterator chain for functional style

#### Bits to String
```rust
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

**Considerations:**
- Handles partial bytes gracefully
- Maintains bit order consistency
- UTF-8 safety concerns addressed

### Algorithm-Specific Data Flow

#### Convolutional Encoding Flow
```
Input Bits → State Update → Polynomial Application → Parity Generation → Output Bits
     ↓              ↓                    ↓                   ↓              ↓
  [1,0,1]    state=0→1→2→5      Apply G1,G2        XOR taps    [1,1,1,0,0,1]
```

#### Cipher Processing Flow
```
Plaintext → Character Processing → Algorithm Application → Ciphertext
    ↓              ↓                      ↓                    ↓
"Hello"    'H','e','l','l','o'    Caesar(+3)/Vigenère/XOR   "Khoor"
```

## Implementation Patterns

### 1. Constructor Patterns

#### Parameter Validation
```rust
impl ConvolutionalEncoder {
    pub fn new(constraint_length: u32, polynomials: Vec<u32>) -> Self {
        // Validate parameters at construction
        assert!(constraint_length >= 2, "Constraint length too small");
        assert!(!polynomials.is_empty(), "Must provide polynomials");

        // Transform data representations
        let binary_polynomials = polynomials.iter()
            .map(|&p| Self::octal_to_binary(p))
            .collect();

        // Initialize with validated parameters
        Self {
            constraint_length,
            polynomials: binary_polynomials,
            rate_denominator: polynomials.len(),
            state: 0,
        }
    }
}
```

### 2. State Management Patterns

#### Encoder State Handling
```rust
pub fn encode(&mut self, input: &[bool]) -> Vec<bool> {
    // State is maintained across encode calls
    for &bit in input {
        // Update state with new input
        self.state = ((self.state << 1) | (bit as u32)) & self.register_mask;
        // Generate outputs based on current state
        // ...
    }
}

pub fn reset(&mut self) {
    // Explicit state reset for independent sequences
    self.state = 0;
}
```

#### Decoder Path Management
```rust
pub fn decode(&self, input: &[bool]) -> Vec<bool> {
    // Stateless operation using local path storage
    let mut paths: HashMap<usize, Path> = HashMap::new();

    // Process input symbols maintaining path history
    for symbol in input.chunks(self.rate_denominator) {
        paths = self.update_paths(paths, symbol);
    }

    // Return best path without modifying decoder state
    self.find_best_path(paths)
}
```

### 3. Algorithm Dispatch Patterns

#### String-Based Algorithm Selection
```rust
impl Cipher {
    pub fn encrypt(&self, message: &str) -> String {
        match self.algorithm.as_str() {
            "caesar" => self.caesar_cipher(message, true),
            "vigenere" => self.vigenere_cipher(message, true),
            "xor" => self.xor_cipher(message),
            _ => panic!("Unsupported algorithm: {}", self.algorithm),
        }
    }
}
```

**Future Enhancement Opportunity:**
```rust
// Trait-based approach for better extensibility
trait CipherAlgorithm {
    fn encrypt(&self, message: &str) -> String;
    fn decrypt(&self, message: &str) -> String;
}

// Registry pattern for dynamic algorithm loading
struct CipherRegistry {
    algorithms: HashMap<String, Box<dyn CipherAlgorithm>>,
}
```

### 4. Error Handling Patterns

#### Current Approach (Panic on Invalid Input)
```rust
pub fn decode(&self, input: &[bool]) -> Vec<bool> {
    if input.len() % self.rate_denominator != 0 {
        panic!("Input length must be multiple of {}", self.rate_denominator);
    }
    // ...
}
```

#### Recommended Future Approach
```rust
pub type Result<T> = std::result::Result<T, CodecError>;

pub fn decode(&self, input: &[bool]) -> Result<Vec<bool>> {
    if input.len() % self.rate_denominator != 0 {
        return Err(CodecError::InvalidInputLength {
            expected_multiple: self.rate_denominator,
            actual: input.len(),
        });
    }
    // ...
}
```

## Extension Points

### Adding New Cipher Algorithms

#### Current Process
1. Add new algorithm method to `Cipher` impl
2. Update `encrypt`/`decrypt` match statements
3. Add CLI parameter validation if needed
4. Implement comprehensive tests

#### Example: Adding Hill Cipher
```rust
impl Cipher {
    fn hill_cipher(&self, message: &str, encrypt: bool) -> String {
        // Parse key as matrix
        let key_matrix = self.parse_hill_key()?;

        // Process message in blocks
        message.chars()
            .collect::<Vec<_>>()
            .chunks(key_matrix.dimension())
            .map(|block| self.hill_transform(block, &key_matrix, encrypt))
            .collect()
    }
}

// Update dispatch
match self.algorithm.as_str() {
    "caesar" => self.caesar_cipher(message, true),
    "vigenere" => self.vigenere_cipher(message, true),
    "hill" => self.hill_cipher(message, true),      // New algorithm
    "xor" => self.xor_cipher(message),
    _ => panic!("Unsupported algorithm: {}", self.algorithm),
}
```

### Adding New Error Correction Codes

#### Current Process
1. Create new module under appropriate domain
2. Implement encoder/decoder structures
3. Add CLI command definitions
4. Update main.rs command routing
5. Add comprehensive tests and documentation

#### Example: Adding Reed-Solomon Codes
```rust
// src/reed_solomon/mod.rs
pub mod encoder;
pub mod decoder;

// src/reed_solomon/encoder.rs
pub struct ReedSolomonEncoder {
    n: usize,           // Codeword length
    k: usize,           // Message length
    generator: Vec<u8>, // Generator polynomial
}

// CLI extension
#[derive(Subcommand)]
pub enum Command {
    // ... existing commands
    RsEncode {
        #[arg(short, long)]
        input: String,

        #[arg(short = 'n', long, default_value_t = 255)]
        codeword_length: usize,

        #[arg(short = 'k', long, default_value_t = 223)]
        message_length: usize,
    },
}
```

### Performance Optimization Extension Points

#### Algorithm-Level Optimizations
```rust
// Current: Direct polynomial evaluation
for &poly in &self.polynomials {
    let parity = self.compute_parity(self.state & poly);
    output.push(parity);
}

// Optimized: Lookup table approach
impl ConvolutionalEncoder {
    fn build_output_table(&self) -> Vec<Vec<bool>> {
        (0..1 << self.constraint_length)
            .map(|state| {
                self.polynomials.iter()
                    .map(|&poly| self.compute_parity(state & poly))
                    .collect()
            })
            .collect()
    }
}
```

#### SIMD Extensions
```rust
// Future: Vectorized bit operations
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

impl ConvolutionalEncoder {
    #[target_feature(enable = "avx2")]
    unsafe fn encode_simd(&mut self, input: &[bool]) -> Vec<bool> {
        // Vectorized polynomial evaluation
        // Process multiple bits in parallel
    }
}
```

## Performance Considerations

### Current Performance Characteristics

#### Time Complexity Analysis
| Algorithm | Operation | Complexity | Notes |
|-----------|-----------|------------|-------|
| Caesar | Encrypt/Decrypt | O(n) | Linear in message length |
| Vigenère | Encrypt/Decrypt | O(n) | Linear with key cycling |
| XOR | Encrypt/Decrypt | O(n) | Linear bitwise operations |
| Convolutional | Encode | O(n × m) | n=length, m=polynomials |
| Viterbi | Decode | O(n × s × m) | s=states, exponential in K |
| Turbo | Encode | O(n × m) | 2× convolutional + interleaving |

#### Memory Usage Analysis
| Component | Memory Usage | Scalability |
|-----------|-------------|-------------|
| Encoder State | O(1) | Constant regardless of input size |
| Viterbi Paths | O(s × n) | Linear in sequence length |
| Interleaver | O(n) | Temporary storage for input permutation |
| Output Buffers | O(n × rate) | Linear with code rate |

### Performance Optimization Strategies

#### 1. Algorithmic Improvements
```rust
// Current: Bit-by-bit processing
for &bit in input {
    self.state = ((self.state << 1) | (bit as u32)) & self.register_mask;
    // Process each bit individually
}

// Optimized: Byte-wise processing
for byte in input.chunks(8) {
    let packed = self.pack_bits(byte);
    let outputs = self.process_byte(packed);
    result.extend(self.unpack_bits(outputs));
}
```

#### 2. Memory Layout Optimizations
```rust
// Current: Vector of booleans (8 bytes per bit)
Vec<bool>

// Optimized: Bit packing
struct BitVec {
    data: Vec<u64>,
    length: usize,
}
```

#### 3. Lookup Table Acceleration
```rust
// Pre-compute common operations
impl ConvolutionalEncoder {
    fn new_optimized(constraint_length: u32, polynomials: Vec<u32>) -> Self {
        let output_table = Self::build_output_table(constraint_length, &polynomials);
        Self {
            constraint_length,
            polynomials,
            output_table, // Pre-computed outputs for each state
            state: 0,
        }
    }
}
```

### Benchmarking Framework

#### Current Testing
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_performance_baseline() {
        let mut encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
        let input = vec![true; 1000];

        let start = std::time::Instant::now();
        let _output = encoder.encode(&input);
        let duration = start.elapsed();

        // Baseline measurement
        println!("Encoded 1000 bits in {:?}", duration);
    }
}
```

#### Proposed Benchmark Suite
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_convolutional_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("convolutional");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("encode", size),
            size,
            |b, &size| {
                let mut encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
                let input = vec![true; size];
                b.iter(|| encoder.encode(black_box(&input)))
            },
        );
    }

    group.finish();
}
```

## Future Architecture Evolution

### Planned Enhancements

#### 1. Error Handling Modernization
```rust
// Current: Panic-based error handling
panic!("Unsupported algorithm: {}", self.algorithm);

// Future: Result-based error handling
pub enum CodecError {
    UnsupportedAlgorithm(String),
    InvalidKeyFormat(String),
    InvalidInputLength { expected: usize, actual: usize },
    DecodingFailure(String),
}

pub type Result<T> = std::result::Result<T, CodecError>;
```

#### 2. Trait-Based Architecture
```rust
// Future: Unified algorithm traits
pub trait Encoder<Input, Output> {
    type Error;

    fn encode(&mut self, input: Input) -> Result<Output, Self::Error>;
    fn reset(&mut self);
}

pub trait Decoder<Input, Output> {
    type Error;

    fn decode(&self, input: Input) -> Result<Output, Self::Error>;
}

// Algorithm-specific implementations
impl Encoder<&[bool], Vec<bool>> for ConvolutionalEncoder {
    type Error = ConvolutionalError;

    fn encode(&mut self, input: &[bool]) -> Result<Vec<bool>, Self::Error> {
        // Implementation
    }
}
```

#### 3. Configuration System
```rust
// Future: Structured configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodecConfig {
    pub convolutional: ConvolutionalConfig,
    pub crypto: CryptoConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone)]
pub struct ConvolutionalConfig {
    pub default_constraint_length: u32,
    pub default_polynomials: Vec<u32>,
    pub soft_decision: bool,
}
```

#### 4. Plugin Architecture
```rust
// Future: Dynamic algorithm loading
pub trait AlgorithmPlugin {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn create_encoder(&self, config: &Config) -> Box<dyn Encoder>;
    fn create_decoder(&self, config: &Config) -> Box<dyn Decoder>;
}

pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn AlgorithmPlugin>>,
}
```

### Migration Strategy

#### Phase 1: Error Handling (v0.2.0)
- Replace panics with Result types
- Implement comprehensive error hierarchy
- Update CLI to handle errors gracefully

#### Phase 2: Trait Unification (v0.3.0)
- Define common traits for algorithms
- Refactor existing implementations
- Maintain backward compatibility

#### Phase 3: Performance Optimization (v0.4.0)
- Implement lookup tables and SIMD
- Add comprehensive benchmarking
- Profile-guided optimizations

#### Phase 4: Plugin System (v1.0.0)
- Dynamic algorithm loading
- Configuration system
- Stable API guarantee

### Compatibility Considerations

#### API Stability
- Semantic versioning for breaking changes
- Deprecation warnings for API changes
- Migration guides for major versions

#### Backward Compatibility
```rust
// Maintain old interfaces during transition
#[deprecated(since = "0.2.0", note = "Use encode_result instead")]
pub fn encode(&mut self, input: &[bool]) -> Vec<bool> {
    self.encode_result(input).unwrap()
}

pub fn encode_result(&mut self, input: &[bool]) -> Result<Vec<bool>> {
    // New implementation
}
```

## Conclusion

The CodeCrypt architecture is designed to balance educational clarity with practical functionality. The modular design allows for independent development of algorithms while maintaining consistent interfaces and patterns.

Key architectural strengths:
- **Clear separation of concerns** between CLI, orchestration, and algorithms
- **Educational focus** with readable implementations
- **Type safety** leveraging Rust's strengths
- **Extensibility** for adding new algorithms

Areas for future improvement:
- **Error handling** modernization
- **Performance optimization** with maintained clarity
- **Plugin architecture** for dynamic extensibility
- **Configuration management** for complex setups

The architecture supports the project's dual goals of education and practical utility while providing a foundation for future enhancements.

## See Also

- [Contributing Guide](contributing.md) - How to extend the architecture
- [API Documentation](../api/) - Detailed interface specifications
- [Performance Analysis](../examples/benchmarks.md) - Current performance characteristics
- [Theory Documentation](../theory/) - Mathematical foundations of the implementations