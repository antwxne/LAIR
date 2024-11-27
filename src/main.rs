pub mod algorithm;

use std::error::Error;

use algorithm::{Algorithm, LuhnAlgorithm};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Key to validate (digits only)
    #[arg()]
    key: String,
}
#[derive(Debug)]
enum ParsingError {
    KeyIsNotAnInterger,
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParsingError::KeyIsNotAnInterger => write!(f, "The key must only contains digits."),
        }
    }
}

impl std::error::Error for ParsingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ParsingError::KeyIsNotAnInterger => None,
        }
    }
}
fn parse_input_key(key: String) -> Result<Vec<u8>, ParsingError> {
    match key.as_bytes().iter().all(|c| c.is_ascii_digit()) {
        true => Ok(key.as_bytes().iter().map(|c| c - '0' as u8).collect()),
        false => Err(ParsingError::KeyIsNotAnInterger),
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let args: Args = Args::parse();

    let digits: Vec<u8> = parse_input_key(args.key)?;
    match LuhnAlgorithm::validate(digits) {
        true => {
            println!("The Key is valid");
            std::process::exit(0);
        }
        false => {
            println!("The Key is not valid");
            std::process::exit(1);
        }
    };
}
