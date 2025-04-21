/**
 * Convolutional Encoder
 * - Constraint Length: K
 * - Generator Polynomials: G1, G2, ..., Gn
 * - Rate Denominator: R = 1/n
 * - State: S
 */
pub struct ConvolutionalEncoder {
    constraint_length: u32,
    polynomials: Vec<u32>,
    rate_denominator: usize,
    state: u32,
}

/**
 * Convolutional Encoder impl
 * - `new`: Create a new encoder with the given constraint length and polynomials
 * - `reset`: Reset the encoder state
 * - `encode`: Encode a sequence of input bits
 * - `get_state`: Get the current state of the encoder
 * - `get_rate`: Get the rate of the encoder
 */
impl ConvolutionalEncoder {
    pub fn new(constraint_length: u32, polynomials: Vec<u32>) -> Self {
        let binary_polynomials = polynomials.iter()
            .map(|&p| {
                let mut binary = 0;
                let mut octal = p;
                let mut shift = 0;
                
                while octal > 0 {
                    let digit = octal % 10;
                    binary |= digit << shift;
                    octal /= 10;
                    shift += 3;
                }
                
                binary
            })
            .collect::<Vec<_>>();
        
        ConvolutionalEncoder {
            constraint_length,
            polynomials: binary_polynomials,
            rate_denominator: polynomials.len(),
            state: 0,
        }
    }
    
    pub fn reset(&mut self) {
        self.state = 0;
    }

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
    
    pub fn get_state(&self) -> u32 {
        self.state
    }
    
    pub fn get_rate(&self) -> (usize, usize) {
        (1, self.rate_denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder() {
        let mut encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
        
        let input = vec![true, false, true, true, false];
        let output = encoder.encode(&input);
        
        let expected = vec![
            true, true,
            true, false,
            false, true,
            true, true,
            true, false,
        ];
        
        assert_eq!(output, expected);
    }
}