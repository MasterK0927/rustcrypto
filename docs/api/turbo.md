# Turbo Module API Documentation

The turbo module implements turbo codes using parallel concatenated convolutional codes (PCCCs).

## Module Structure

```rust
pub mod encoder;
```

## TurboEncoder

### Overview

The `TurboEncoder` implements a systematic turbo encoder using two parallel convolutional encoders with an interleaver between them.

### Declaration

```rust
pub struct TurboEncoder {
    encoder1: ConvolutionalEncoder,
    encoder2: ConvolutionalEncoder,
}
```

### Architecture

```
Input → [Encoder 1] → Parity 1
  ↓
  → [Interleaver] → [Encoder 2] → Parity 2
  ↓
Systematic Output

Final Output: [Systematic | Parity 1 | Parity 2]
```

### Methods

#### `new(polynomials: Vec<u32>) -> Self`

Creates a new turbo encoder with identical constituent encoders.

**Parameters:**
- `polynomials`: Generator polynomials for both constituent encoders

**Returns:** New `TurboEncoder` instance

**Implementation:**
```rust
pub fn new(polynomials: Vec<u32>) -> Self {
    TurboEncoder {
        encoder1: ConvolutionalEncoder::new(3, polynomials.clone()),
        encoder2: ConvolutionalEncoder::new(3, polynomials),
    }
}
```

**Example:**
```rust
let turbo_encoder = TurboEncoder::new(vec![7, 5]);
// Creates turbo encoder with constituent encoders using polynomials (111₂, 101₂)
```

#### `encode(&mut self, input: &[bool]) -> Vec<bool>`

Encodes input data using the turbo coding scheme.

**Parameters:**
- `input`: Slice of boolean values representing input bits

**Returns:** Systematic turbo-encoded output with format `[systematic | parity1 | parity2]`

**Algorithm:**
1. Reset both constituent encoders
2. Encode input with first encoder → parity1
3. Interleave input using algebraic interleaver
4. Encode interleaved input with second encoder → parity2
5. Concatenate: systematic + parity1 + parity2

**Example:**
```rust
let mut encoder = TurboEncoder::new(vec![7, 5]);
let input = vec![true, false, true, true, false];
let encoded = encoder.encode(&input);

// Output structure:
// [systematic bits] + [parity from encoder1] + [parity from encoder2]
// Length: input.len() + 2 * input.len() * polynomials.len()
```

**Implementation:**
```rust
pub fn encode(&mut self, input: &[bool]) -> Vec<bool> {
    // Reset encoders
    self.encoder1.reset();
    self.encoder2.reset();

    // Encode with first encoder
    let encoded1 = self.encoder1.encode(input);

    // Interleave input
    let interleaved = self.interleave(input);

    // Encode interleaved input with second encoder
    let encoded2 = self.encoder2.encode(&interleaved);

    // Combine outputs: systematic bits, parity bits from encoder 1, parity bits from encoder 2
    let mut output = input.to_vec();
    output.extend_from_slice(&encoded1);
    output.extend_from_slice(&encoded2);

    output
}
```

#### `interleave(&self, input: &[bool]) -> Vec<bool>` (Private)

Applies algebraic interleaving to randomize the input sequence.

**Parameters:**
- `input`: Input bit sequence

**Returns:** Interleaved bit sequence

**Algorithm:**
Uses simple algebraic interleaver: `new_position = (i * 7 + 5) % length`

**Implementation:**
```rust
fn interleave(&self, input: &[bool]) -> Vec<bool> {
    let len = input.len();

    // Use a simple permutation: i -> (i * 7 + 5) % len
    let mut interleaved = vec![false; len];
    for i in 0..len {
        let new_pos = (i * 7 + 5) % len;
        interleaved[new_pos] = input[i];
    }
    interleaved
}
```

**Interleaver Properties:**
- **Algebraic**: Deterministic permutation pattern
- **Invertible**: Can be reversed for decoding
- **Spread**: Distributes adjacent bits across the sequence
- **Efficient**: O(n) time complexity

### Output Format

The turbo encoder produces systematic output with the following structure:

```
Total Length = input_length + 2 * (input_length * rate_denominator)

┌─────────────┬─────────────┬─────────────┐
│ Systematic  │   Parity 1  │   Parity 2  │
│    Bits     │  (Encoder1) │  (Encoder2) │
└─────────────┴─────────────┴─────────────┘
```

**For rate 1/2 constituent encoders:**
- Input: 5 bits
- Systematic: 5 bits
- Parity 1: 10 bits (5 × 2)
- Parity 2: 10 bits (5 × 2)
- Total: 25 bits

### Usage Patterns

#### Basic Turbo Encoding
```rust
let mut encoder = TurboEncoder::new(vec![7, 5]);
let message = vec![true, false, true, true, false];
let encoded = encoder.encode(&message);

println!("Original length: {}", message.len());
println!("Encoded length: {}", encoded.len());
// Encoded length = 5 + 10 + 10 = 25 (for rate 1/2 constituent encoders)
```

#### Different Polynomial Configurations
```rust
// Standard polynomials
let encoder1 = TurboEncoder::new(vec![7, 5]);    // (111₂, 101₂)
let encoder2 = TurboEncoder::new(vec![5, 7]);    // (101₂, 111₂)

// Higher rate
let encoder3 = TurboEncoder::new(vec![7, 5, 3]); // Rate 1/3 constituent encoders
```

#### Systematic Property Verification
```rust
let mut encoder = TurboEncoder::new(vec![7, 5]);
let input = vec![true, false, true];
let output = encoder.encode(&input);

// First bits of output should match input (systematic property)
for i in 0..input.len() {
    assert_eq!(output[i], input[i]);
}
```

### Interleaver Design

#### Algebraic Interleaver
The current implementation uses a simple algebraic interleaver:

```rust
new_position = (i * 7 + 5) % length
```

**Properties:**
- **Deterministic**: Same input always produces same interleaving
- **Full Permutation**: Each position maps to exactly one other position
- **Relatively Prime**: Constants 7 and length should be relatively prime for best performance

#### Alternative Interleaver Patterns
```rust
// Current: Algebraic
new_pos = (i * 7 + 5) % len;

// Alternative 1: Block interleaver
new_pos = (i % rows) * cols + (i / rows);

// Alternative 2: Pseudo-random
new_pos = prng_sequence[i];
```

### Performance Characteristics

#### Time Complexity
- **Encoding**: O(3n × m) where n = input length, m = polynomials per encoder
- **Interleaving**: O(n)

#### Space Complexity
- **Memory**: O(n) for interleaved sequence storage
- **Output**: O(n × (1 + 2m)) where m = rate denominator

#### Code Rate
For constituent encoders with rate 1/m:
```
Overall turbo code rate = input_bits / total_output_bits
                       = n / (n + 2nm)
                       = 1 / (1 + 2m)

Examples:
- Rate 1/2 constituents → Turbo rate 1/3
- Rate 1/3 constituents → Turbo rate 1/7
```

### Advanced Usage

#### Multiple Block Encoding
```rust
let mut encoder = TurboEncoder::new(vec![7, 5]);

let blocks = vec![
    vec![true, false, true],
    vec![false, true, false, true],
    vec![true, true, false],
];

for block in blocks {
    let encoded = encoder.encode(&block);
    // Process each block independently
}
```

#### Rate Calculation
```rust
let mut encoder = TurboEncoder::new(vec![7, 5]); // Rate 1/2 constituents
let input = vec![true; 100]; // 100 bits
let output = encoder.encode(&input);

let code_rate = input.len() as f64 / output.len() as f64;
println!("Code rate: {:.3}", code_rate); // Should be ~0.333 (1/3)
```

### Error Correction Capability

Turbo codes provide excellent error correction performance:

- **Iterative Decoding**: Uses message passing between constituent decoders
- **Near Shannon Limit**: Approaches theoretical channel capacity
- **Burst Error Resilience**: Interleaver spreads burst errors

### Testing

The module includes unit tests to verify functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turbo_encoder() {
        let mut encoder = TurboEncoder::new(vec![7, 5]);
        let input = vec![true, false, true, true, false];
        let output = encoder.encode(&input);

        // Output should be longer than input
        assert!(output.len() > input.len());

        // First bits should match input (systematic property)
        for i in 0..input.len() {
            assert_eq!(output[i], input[i]);
        }
    }

    #[test]
    fn test_systematic_property() {
        let mut encoder = TurboEncoder::new(vec![7, 5]);
        let input = vec![true, false, true];
        let output = encoder.encode(&input);

        // Verify systematic property
        assert_eq!(&output[..input.len()], &input[..]);
    }
}
```

### Integration with Other Modules

#### With Convolutional Encoder
```rust
use crate::convolutional::encoder::ConvolutionalEncoder;

// TurboEncoder internally uses two ConvolutionalEncoders
let turbo = TurboEncoder::new(vec![7, 5]);
// Equivalent to:
let conv1 = ConvolutionalEncoder::new(3, vec![7, 5]);
let conv2 = ConvolutionalEncoder::new(3, vec![7, 5]);
```

#### Output Processing
```rust
let mut encoder = TurboEncoder::new(vec![7, 5]);
let input_bits = "Hello".chars()
    .flat_map(|c| {
        let byte = c as u8;
        (0..8).map(move |i| (byte >> (7 - i)) & 1 == 1)
    })
    .collect::<Vec<_>>();

let encoded = encoder.encode(&input_bits);

// Convert back to bytes for transmission/storage
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

## Future Enhancements

### Planned Features
1. **Turbo Decoder**: BCJR algorithm implementation
2. **Advanced Interleavers**: Block, convolutional, and optimized patterns
3. **Puncturing**: Rate-compatible punctured turbo codes
4. **Parallel Processing**: Multi-threaded encoding for large blocks

### Optimization Opportunities
1. **SIMD Instructions**: Vectorized bit operations
2. **Custom Interleavers**: Problem-specific interleaving patterns
3. **Memory Pool**: Reuse allocated memory for repeated encoding
4. **Lookup Tables**: Pre-computed encoder outputs

## See Also

- [Turbo Codes Theory](../theory/turbo-codes.md)
- [Convolutional API](convolutional.md)
- [Turbo Tutorial](../tutorials/convolutional-tutorial.md#turbo-codes)
- [Performance Benchmarks](../examples/benchmarks.md#turbo-encoding)