//! Ed25519 signature operations.
//!
//! All signing is performed over **raw bytes** (typically 32-byte Blake3 hashes),
//! never over hex-encoded strings.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use arxia_core::{AccountId, ArxiaError, SignatureBytes};

/// Generate a new Ed25519 keypair.
pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

/// Sign raw bytes with an Ed25519 signing key.
///
/// The `data` parameter must be raw bytes (e.g., a 32-byte Blake3 hash),
/// NOT a hex string.
pub fn sign(signing_key: &SigningKey, data: &[u8]) -> SignatureBytes {
    signing_key.sign(data).to_bytes()
}

/// Verify an Ed25519 signature over raw bytes.
pub fn verify(
    pubkey: &AccountId,
    data: &[u8],
    signature: &SignatureBytes,
) -> Result<(), ArxiaError> {
    let vk = VerifyingKey::from_bytes(pubkey).map_err(|e| ArxiaError::InvalidKey(e.to_string()))?;
    let sig = Signature::from_bytes(signature);
    vk.verify(data, &sig)
        .map_err(|e| ArxiaError::SignatureInvalid(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify_raw_bytes() {
        let (sk, vk) = generate_keypair();
        let data = crate::hash_blake3_bytes(b"test message");
        let sig = sign(&sk, &data);
        assert!(verify(&vk.to_bytes(), &data, &sig).is_ok());
    }

    #[test]
    fn test_verify_rejects_tampered_data() {
        let (sk, vk) = generate_keypair();
        let data = crate::hash_blake3_bytes(b"original");
        let sig = sign(&sk, &data);
        let tampered = crate::hash_blake3_bytes(b"tampered");
        assert!(verify(&vk.to_bytes(), &tampered, &sig).is_err());
    }

    #[test]
    fn test_verify_rejects_wrong_key() {
        let (sk, _) = generate_keypair();
        let (_, vk2) = generate_keypair();
        let data = [0u8; 32];
        let sig = sign(&sk, &data);
        assert!(verify(&vk2.to_bytes(), &data, &sig).is_err());
    }

    #[test]
    fn test_sign_over_blake3_bytes_not_hex() {
        let (sk, vk) = generate_keypair();
        let hash_hex = crate::hash_blake3(b"important");
        let hash_bytes = hex::decode(&hash_hex).unwrap();
        assert_eq!(hash_bytes.len(), 32);
        let sig = sign(&sk, &hash_bytes);
        assert!(verify(&vk.to_bytes(), &hash_bytes, &sig).is_ok());
        let wrong_sig = sign(&sk, hash_hex.as_bytes());
        assert!(verify(&vk.to_bytes(), &hash_bytes, &wrong_sig).is_err());
    }

    #[test]
    fn test_generate_keypair_unique() {
        let (_, vk1) = generate_keypair();
        let (_, vk2) = generate_keypair();
        assert_ne!(vk1.to_bytes(), vk2.to_bytes());
    }
}
