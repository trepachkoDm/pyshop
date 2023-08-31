mod args_parser;
mod hash_computer;
mod hash_finder;

use args_parser::parse_args;
use hash_finder::find_hashes;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match parse_args(&args) {
        Some((n, f)) => {
            let hashes = find_hashes(n, f);
            for (num, hash) in hashes.iter() {
                println!("{}, \"{}\"", num, hash);
            }
        }
        None => {
            println!("Usage: ./hash_finder -N <number of zeros> -F <number of hashes to find>");
        }
    }
}
