use clap::{Parser, Subcommand};

/**
 * Cli 
 * - command: The command to run
 */
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/**
 * Command
 * - conv_encode: Convolutional encoder
 * - conv_decode: Convolutional decoder
 * - turbo_encode: Turbo encoder
 * - encrypt: Encrypt a message
 * - decrypt: Decrypt a message
 */
#[derive(Subcommand)]
pub enum Command {
    ConvEncode {
        #[arg(short, long)]
        input: String,
        
        #[arg(short, long, num_args = 1.., value_parser = clap::value_parser!(u32))]
        polynomials: Vec<u32>,
        
        #[arg(short, long, default_value_t = 3)]
        constraint_length: u32,
    },
    
    ConvDecode {
        #[arg(short, long)]
        input: String,
        
        #[arg(short, long, num_args = 1.., value_parser = clap::value_parser!(u32))]
        polynomials: Vec<u32>,
        
        #[arg(short, long, default_value_t = 3)]
        constraint_length: u32,
    },
    
    TurboEncode {
        #[arg(short, long)]
        input: String,
        
        #[arg(short, long, num_args = 1.., value_parser = clap::value_parser!(u32))]
        polynomials: Vec<u32>,
    },
    
    Encrypt {
        #[arg(short, long)]
        input: String,
        
        #[arg(short, long)]
        key: String,
        
        #[arg(short, long, default_value = "xor")]
        algorithm: String,
    },
    
    Decrypt {
        #[arg(short, long)]
        input: String,
        
        #[arg(short, long)]
        key: String,
        
        #[arg(short, long, default_value = "xor")]
        algorithm: String,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}