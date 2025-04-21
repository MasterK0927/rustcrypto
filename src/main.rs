mod cli;
mod convolutional;
mod turbo;
mod crypto;

use cli::Command;

/**
 * - Main function to parse command line arguments and execute the corresponding command.
 * - It uses the clap library for argument parsing and handles different commands
 *   such as convolutional encoding/decoding, turbo encoding, and encryption/decryption.
 * - Each command is implemented in its respective module.
 * - The main function is the entry point of the program.
 * - It initializes the command line interface, parses the arguments, and executes the command.
 * - The program is designed to be extensible, allowing for the addition of new commands and features in the future.
 * - The commands include convolutional encoding/decoding, turbo encoding, and encryption/decryption.
 */
fn main() {
    let cli = cli::parse_args();
    
    match cli.command {
        Command::ConvEncode { input, polynomials, constraint_length } => {
            let mut encoder = convolutional::encoder::ConvolutionalEncoder::new(
                constraint_length, 
                polynomials
            );
            
            let input_bits = input.chars()
                .flat_map(|c| {
                    let byte = c as u8;
                    (0..8).map(move |i| (byte >> (7 - i)) & 1 == 1)
                })
                .collect::<Vec<_>>();
            
            let encoded = encoder.encode(&input_bits);
            
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
            
            println!("Encoded: {}", result);
        },
        
        Command::ConvDecode { input, polynomials, constraint_length } => {
            let decoder = convolutional::viterbi::ViterbiDecoder::new(
                constraint_length,
                polynomials
            );
            
            let input_bits = input.chars()
                .flat_map(|c| {
                    let byte = c as u8;
                    (0..8).map(move |i| (byte >> (7 - i)) & 1 == 1)
                })
                .collect::<Vec<_>>();
            
            let decoded = decoder.decode(&input_bits);
            
            let result = decoded.chunks(8)
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
                
            println!("Decoded: {}", result);
        },
        
        Command::TurboEncode { input, polynomials } => {
            let mut encoder = turbo::encoder::TurboEncoder::new(polynomials);
            
            let input_bits = input.chars()
                .flat_map(|c| {
                    let byte = c as u8;
                    (0..8).map(move |i| (byte >> (7 - i)) & 1 == 1)
                })
                .collect::<Vec<_>>();
            
            let encoded = encoder.encode(&input_bits);
            
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
                
            println!("Turbo Encoded: {}", result);
        },
        
        Command::Encrypt { input, key, algorithm } => {
            let cipher = crypto::cipher::Cipher::new(algorithm, key);
            let encrypted = cipher.encrypt(&input);
            println!("Encrypted: {}", encrypted);
        },
        
        Command::Decrypt { input, key, algorithm } => {
            let cipher = crypto::cipher::Cipher::new(algorithm, key);
            let decrypted = cipher.decrypt(&input);
            println!("Decrypted: {}", decrypted);
        },
    }
}