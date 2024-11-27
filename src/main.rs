pub mod algorithm;
use algorithm::{Algorithm, LuhnAlgorithm};

fn main() {
    let mut args = std::env::args();
    let digits: Vec<u8> = args
        .nth(1)
        .expect("Usage: rlai <key>")
        .as_bytes()
        .iter()
        .map(|c| c - '0' as u8)
        .collect();

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
