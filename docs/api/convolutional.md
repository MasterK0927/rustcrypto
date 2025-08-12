# Convolutional Module API Documentation

The convolutional module provides implementations for convolutional encoding and Viterbi decoding.

## Module Structure

```rust
pub mod encoder;
pub mod viterbi;
```

## ConvolutionalEncoder

### Overview

The `ConvolutionalEncoder` implements a rate-1/n convolutional encoder with configurable constraint length and generator polynomials.

### Declaration

```rust
pub struct ConvolutionalEncoder {
    constraint_length: u32,
    polynomials: Vec<u32>,
    rate_denominator: usize,
    state: u32,
}
```

### Methods

#### `new(constraint_length: u32, polynomials: Vec<u32>) -> Self`

Creates a new convolutional encoder.

**Parameters:**
- `constraint_length`: Memory length of the encoder (K)
- `polynomials`: Generator polynomials in octal notation

**Returns:** New `ConvolutionalEncoder` instance

**Example:**
```rust
let encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
// Creates encoder with constraint length 3 and polynomials 111₂, 101₂
```

**Polynomial Conversion:**
The constructor automatically converts octal polynomials to binary:
- Input: `7` (octal) → `111₂` (binary) → `7` (decimal)
- Input: `5` (octal) → `101₂` (binary) → `5` (decimal)

#### `reset(&mut self)`

Resets the encoder state to zero.

**Example:**
```rust
encoder.reset(); // State becomes 0
```

#### `encode(&mut self, input: &[bool]) -> Vec<bool>`

Encodes a sequence of input bits.

**Parameters:**
- `input`: Slice of boolean values representing input bits

**Returns:** Vector of encoded bits with length `input.len() * rate_denominator`

**Algorithm:**
1. Shift input bit into encoder state
2. Apply each generator polynomial via XOR
3. Calculate parity for each polynomial
4. Output parity bits in order

**Example:**
```rust
let input = vec![true, false, true];
let encoded = encoder.encode(&input);
// For rate 1/2: output length = 3 * 2 = 6 bits
```

**Implementation Detail:**
```rust
// State update: shift left and mask
self.state = ((self.state << 1) | (if bit { 1 } else { 0 })) & register_mask;

// Parity calculation for each polynomial
for &poly in &self.polynomials {
    let mut parity = 0;
    let mut temp = self.state & poly;

    while temp != 0 {
        parity ^= temp & 1;
        temp >>= 1;
    }

    output.push(parity == 1);
}
```

### Usage Patterns

#### Basic Encoding
```rust
let mut encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
let message = vec![true, false, true, true, false];
let encoded = encoder.encode(&message);
```

#### Rate Configuration
```rust
// Rate 1/2 encoder
let encoder_12 = ConvolutionalEncoder::new(3, vec![7, 5]);

// Rate 1/3 encoder
let encoder_13 = ConvolutionalEncoder::new(3, vec![7, 5, 3]);
```

#### Common Polynomial Sets
```rust
// NASA standard polynomials
let k3_75 = ConvolutionalEncoder::new(3, vec![7, 5]);     // (111, 101)
let k4_1517 = ConvolutionalEncoder::new(4, vec![15, 17]); // (1111, 1101)
let k5_2335 = ConvolutionalEncoder::new(5, vec![23, 35]); // (10011, 11101)
```

## ViterbiDecoder

### Overview

The `ViterbiDecoder` implements maximum likelihood sequence estimation for convolutional codes using the Viterbi algorithm.

### Declaration

```rust
pub struct ViterbiDecoder {
    constraint_length: u32,
    polynomials: Vec<u32>,
    num_states: usize,
    rate_denominator: usize,
}
```

### Internal Structures

#### Path Structure
```rust
struct Path {
    bits: Vec<bool>,    // Decoded bit sequence
    metric: usize,      // Accumulated path metric
}
```

### Methods

#### `new(constraint_length: u32, polynomials: Vec<u32>) -> Self`

Creates a new Viterbi decoder with matching encoder parameters.

**Parameters:**
- `constraint_length`: Must match encoder constraint length
- `polynomials`: Must match encoder generator polynomials

**Returns:** New `ViterbiDecoder` instance

**Example:**
```rust
let decoder = ViterbiDecoder::new(3, vec![7, 5]);
```

#### `decode(&self, input: &[bool]) -> Vec<bool>`

Decodes a sequence of received bits using the Viterbi algorithm.

**Parameters:**
- `input`: Encoded bit sequence (length must be multiple of rate denominator)

**Returns:** Decoded bit sequence

**Algorithm Steps:**
1. **Initialize**: Start with single path at state 0
2. **Branch Extension**: For each received symbol, extend all paths
3. **Metric Calculation**: Compute Hamming distance for each branch
4. **Path Selection**: Keep best path to each state
5. **Traceback**: Select path with minimum final metric

**Example:**
```rust
let received = vec![true, true, false, true, true, false];
let decoded = decoder.decode(&received);
```

#### `compute_output(&self, state: usize, input_bit: bool) -> Vec<bool>`

Computes expected encoder output for given state and input bit.

**Parameters:**
- `state`: Current encoder state
- `input_bit`: Input bit (0 or 1)

**Returns:** Expected output bits

**Usage:** Internal method for branch metric calculation.

#### `hamming_distance(a: &[bool], b: &[bool]) -> usize`

Calculates Hamming distance between two bit sequences.

**Parameters:**
- `a`: First bit sequence
- `b`: Second bit sequence

**Returns:** Number of differing bits

**Example:**
```rust
let dist = ViterbiDecoder::hamming_distance(&[true, false], &[true, true]);
// Returns: 1
```

### Algorithm Implementation Details

#### Trellis Structure
The decoder maintains a trellis with `2^(K-1)` states where K is the constraint length.

```rust
let num_states = 1 << (constraint_length - 1);
```

#### Path Metrics
Uses accumulated Hamming distance as the path metric:
```rust
let branch_metric = Self::hamming_distance(received_symbol, expected_output);
let new_metric = path.metric + branch_metric;
```

#### State Management
Efficient path storage using HashMap:
```rust
let mut paths: HashMap<usize, Path> = HashMap::new();
```

### Usage Patterns

#### Basic Decoding
```rust
let decoder = ViterbiDecoder::new(3, vec![7, 5]);
let received = encoded_bits; // From encoder output
let decoded = decoder.decode(&received);
```

#### Error Handling
```rust
// Input length validation
if input.len() % self.rate_denominator != 0 {
    panic!("Input length must be multiple of {}", self.rate_denominator);
}
```

#### Round-trip Verification
```rust
let mut encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
let decoder = ViterbiDecoder::new(3, vec![7, 5]);

let original = vec![true, false, true, true];
let encoded = encoder.encode(&original);
let decoded = decoder.decode(&encoded);

assert_eq!(original, decoded);
```

## Performance Considerations

### Time Complexity
- **Encoder**: O(n × m) where n = input length, m = number of polynomials
- **Decoder**: O(n × s × m) where s = number of states

### Space Complexity
- **Encoder**: O(1) state storage
- **Decoder**: O(s × n) for path storage

### Optimization Tips

1. **Polynomial Selection**: Use standard polynomial sets for better performance
2. **Batch Processing**: Process longer sequences for better amortization
3. **Memory Management**: Reset encoder state between independent sequences

## Testing

The module includes comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder() {
        let mut encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
        let input = vec![true, false, true, true, false];
        let output = encoder.encode(&input);

        let expected = vec![
            true, true,   // First bit
            true, false,  // Second bit
            false, true,  // Third bit
            true, true,   // Fourth bit
            true, false,  // Fifth bit
        ];

        assert_eq!(output, expected);
    }

    #[test]
    fn test_viterbi_decode() {
        let mut encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
        let message = vec![true, false, true, true, false, true, false, true];
        let encoded = encoder.encode(&message);

        let decoder = ViterbiDecoder::new(3, vec![7, 5]);
        let decoded = decoder.decode(&encoded);

        assert_eq!(message, decoded);
    }
}
```

## Common Issues and Solutions

### Issue: Incorrect Polynomial Format
**Problem:** Using binary instead of octal notation
```rust
// ❌ Wrong
ConvolutionalEncoder::new(3, vec![0b111, 0b101]);

// ✅ Correct
ConvolutionalEncoder::new(3, vec![7, 5]);
```

### Issue: Mismatched Parameters
**Problem:** Decoder parameters don't match encoder
```rust
// ❌ Wrong
let encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
let decoder = ViterbiDecoder::new(4, vec![15, 17]); // Different parameters!

// ✅ Correct
let encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
let decoder = ViterbiDecoder::new(3, vec![7, 5]); // Matching parameters
```

### Issue: Invalid Input Length
**Problem:** Input to decoder not multiple of rate denominator
```rust
// For rate 1/2 encoder, decoder expects even number of bits
let encoded = vec![true, false, true]; // Length 3, not multiple of 2
let decoded = decoder.decode(&encoded); // ❌ Panics!
```

## See Also

- [Convolutional Codes Theory](../theory/convolutional-codes.md)
- [Viterbi Algorithm Theory](../theory/viterbi-algorithm.md)
- [Convolutional Tutorial](../tutorials/convolutional-tutorial.md)