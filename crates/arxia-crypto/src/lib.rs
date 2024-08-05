//! Cryptographic primitives for the Arxia protocol.
//!
//! Provides Ed25519 signatures, Blake3 hashing, ChaCha20-Poly1305
//! encryption, and SLIP39 seed backup.
//!
//! # Critical invariant
//!
//! Ed25519 signatures are computed over **raw Blake3 bytes** (32 bytes),
//! NOT over the hex-encoded string (64 bytes). This was a bug fixed in
//! PoC v0.3.0 and must never be reintroduced.

#![deny(unsafe_code)]
#![warn(missing_docs)]

pub mod blake3_hash;
pub mod chacha20;
pub mod ed25519;
pub mod slip39;

pub use blake3_hash::{hash_blake3, hash_blake3_bytes};
pub use ed25519::{generate_keypair, sign, verify};
