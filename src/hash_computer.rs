use sha2::{Digest, Sha256};

// This code defines the compute_hash function, which takes an integer of type usize and returns its hash as a string calculated using the SHA-256 algorithm.

pub fn compute_hash(num: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(num.to_string());
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_hash() {
        let hash = compute_hash(1);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_compute_hash_length() {
        let hash = compute_hash(1);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_compute_hash_uniqueness() {
        let hash1 = compute_hash(1);
        let hash2 = compute_hash(2);
        assert_ne!(hash1, hash2);
    }
}
