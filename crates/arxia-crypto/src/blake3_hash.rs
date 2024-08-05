//! Blake3 hashing utilities.

/// Compute Blake3 hash and return the hex-encoded string.
pub fn hash_blake3(data: &[u8]) -> String {
    let hash = blake3::hash(data);
    hash.to_hex().to_string()
}

/// Compute Blake3 hash and return raw 32-byte array.
pub fn hash_blake3_bytes(data: &[u8]) -> [u8; 32] {
    *blake3::hash(data).as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_hash_deterministic() {
        let a = hash_blake3(b"hello");
        let b = hash_blake3(b"hello");
        assert_eq!(a, b);
    }

    #[test]
    fn test_blake3_hash_different_inputs() {
        let a = hash_blake3(b"hello");
        let b = hash_blake3(b"world");
        assert_ne!(a, b);
    }

    #[test]
    fn test_blake3_hex_length() {
        let h = hash_blake3(b"test");
        assert_eq!(h.len(), 64);
    }

    #[test]
    fn test_blake3_bytes_length() {
        let h = hash_blake3_bytes(b"test");
        assert_eq!(h.len(), 32);
    }

    #[test]
    fn test_blake3_hex_and_bytes_consistent() {
        let data = b"consistency check";
        let hex_str = hash_blake3(data);
        let bytes = hash_blake3_bytes(data);
        assert_eq!(hex::encode(bytes), hex_str);
    }
}
