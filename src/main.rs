// use sha2::{Digest, Sha256};
// use std::env;
// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let mut n = 0;
//     let mut f = 0;

//     for i in 1..args.len() {
//         if args[i] == "-N" {
//             if i + 1 < args.len() {
//                 n = args[i + 1].parse::<usize>().expect("Invalid value for N");
//             }
//         } else if args[i] == "-F" && i + 1 < args.len() {
//             f = args[i + 1].parse::<usize>().expect("Invalid value for F");
//         }
//     }

//     if n == 0 || f == 0 {
//         println!("Usage: ./hash_finder -N <number of zeros> -F <number of hashes to find>");
//         return;
//     }

//     let counter = Arc::new(Mutex::new(1));
//     let found_hashes = Arc::new(Mutex::new(Vec::new()));

//     let num_threads = 8;
//     let mut handles = vec![];

//     for _ in 0..num_threads {
//         let counter = Arc::clone(&counter);
//         let found_hashes = Arc::clone(&found_hashes);

//         let handle = thread::spawn(move || loop {
//             let num;
//             {
//                 let mut counter = counter.lock().expect("Failed to lock counter");
//                 num = *counter;
//                 *counter += 1;
//             }

//             let mut hasher = Sha256::new();
//             hasher.update(num.to_string());
//             let result = hasher.finalize();
//             let hash_str = format!("{:x}", result);

//             if hash_str.ends_with(&"0".repeat(n)) {
//                 let mut found_hashes = found_hashes.lock().expect("Failed to lock found_hashes");
//                 found_hashes.push((num, hash_str));

//                 if found_hashes.len() >= f {
//                     break;
//                 }
//             }
//         });

//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     let found_hashes = found_hashes.lock().unwrap();
//     for (num, hash) in found_hashes.iter() {
//         println!("{}, \"{}\"", num, hash);
//     }
// }

// _______________________      _____________________      ____________________ //

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
