use crate::convolutional::encoder::ConvolutionalEncoder;

/**
 * TurboEncoder
 * - encoder1: The first convolutional encoder
 * - encoder2: The second convolutional encoder
 */
pub struct TurboEncoder {
    encoder1: ConvolutionalEncoder,
    encoder2: ConvolutionalEncoder,
}

/**
 * TurboEncoder constructor
 * - new: Creates a new TurboEncoder with the given polynomials, takes a vector of u32
 * - interleave: Interleaves the input bits using a simple permutation, i -> (i * 7 + 5) % len
 * - encode: Encodes the input bits using the two convolutional encoders, interleaving the input for the second encoder
 * - Returns: The encoded bits, which include the systematic bits and parity bits from both encoders
 */
impl TurboEncoder {

    pub fn new(polynomials: Vec<u32>) -> Self {
        TurboEncoder {
            encoder1: ConvolutionalEncoder::new(3, polynomials.clone()),
            encoder2: ConvolutionalEncoder::new(3, polynomials),
        }
    }
    
    /**
     * interleave
     * - input: The input bits to interleave
     * - Returns: The interleaved bits
     * - Simple permutation: i -> (i * 7 + 5) % len
     */
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
    
    /**
     * encode
     * - input: The input bits to encode
     * - Resets the encoders before encoding
     * - Encodes the input with the first encoder
     * - Interleaves the input and encodes it with the second encoder
     * - Combines the outputs: systematic bits, parity bits from encoder 1, parity bits from encoder 2
     * - Returns the final encoded bits
     */
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turbo_encoder() {
        let mut encoder = TurboEncoder::new(vec![7, 5]);
        let input = vec![true, false, true, true, false];
        let output = encoder.encode(&input);
        assert!(output.len() > input.len());
        
        for i in 0..input.len() {
            assert_eq!(output[i], input[i]);
        }
    }
}