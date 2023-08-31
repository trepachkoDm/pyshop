use crate::hash_computer::compute_hash;
use std::sync::{Arc, Mutex};
use std::thread;

// This code defines the find_hashes function, which finds hashes with a given number of zeros at the end.
// It works in a multithreaded environment using 8 threads to speed up the process.

pub fn find_hashes(n: usize, f: usize) -> Vec<(usize, String)> {
    let counter = Arc::new(Mutex::new(1));
    let found_hashes = Arc::new(Mutex::new(Vec::new()));
    let num_threads = 8;
    let mut handles = vec![];

    // The loop runs threads that will search for hashes.
    for _ in 0..num_threads {
        let counter = Arc::clone(&counter);
        let found_hashes = Arc::clone(&found_hashes);

        // Flow logic: Each thread in the loop calculates hashes and checks if they match the given conditions.
        // If so, they are added to found_hashes.
        let handle = thread::spawn(move || loop {
            let num;
            // Lock counter and increment
            {
                let mut counter = counter.lock().expect("Failed to lock counter");
                num = *counter;
                *counter += 1;
            }

            // Compute hash
            let hash = compute_hash(num);

            // Check condition
            if hash.ends_with(&"0".repeat(n)) {
                let mut found_hashes = found_hashes.lock().expect("Failed to lock found_hashes");

                // Add to found_hashes if condition is met
                if found_hashes.len() < f {
                    found_hashes.push((num, hash));
                }

                // Break if enough hashes are found
                if found_hashes.len() >= f {
                    break;
                }
            }
        });

        handles.push(handle);
    }

    // Waiting for threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Return the hashes found
    let found_hashes = found_hashes.lock().unwrap();
    found_hashes.clone()
}

#[cfg(test)]
mod tests {
    use super::find_hashes;
    // Checking for the correctness of finding the specified number of hashes with a given number of zeros at the end.
    #[test]
    fn test_find_hashes() {
        let hashes = find_hashes(1, 2);
        assert_eq!(hashes.len(), 2);
        for (_, hash) in &hashes {
            assert!(hash.ends_with('0'));
        }
    }

    // Checking that all found hashes are unique.
    #[test]
    fn test_find_hashes_unique() {
        let hashes = find_hashes(1, 10);
        let unique_hashes: std::collections::HashSet<_> = hashes.iter().collect();
        assert_eq!(hashes.len(), unique_hashes.len());
    }

    // Checking that the numbers for which hashes were found are in ascending order.
    #[test]
    fn test_find_hashes_sorted() {
        let hashes = find_hashes(1, 10);
        let mut last_num = 0;
        for (num, _) in hashes.iter() {
            assert!(*num > last_num);
            last_num = *num;
        }
    }

    // Checking that the function works correctly even if a very large number of hashes are specified.
    #[test]
    #[ignore] // This test can take a long time
    fn test_find_hashes_large_f() {
        let hashes = find_hashes(1, 1000);
        assert_eq!(hashes.len(), 1000);
    }

    // Hash format check: Each hash found is in the correct format (e.g. 64 characters long for SHA256).
    #[test]
    fn test_find_hashes_format() {
        let hashes = find_hashes(1, 10);
        for (_, hash) in hashes.iter() {
            assert_eq!(hash.len(), 64);
        }
    }
}
