//! Block and BlockType definitions for the Arxia Block Lattice.

use serde::{Deserialize, Serialize};

/// The type of operation a block represents.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BlockType {
    /// Genesis block opening an account with an initial balance.
    Open {
        /// Initial balance in micro-ARX.
        initial_balance: u64,
    },
    /// Send funds to another account.
    Send {
        /// Destination account public key (hex-encoded).
        destination: String,
        /// Amount in micro-ARX.
        amount: u64,
    },
    /// Receive funds from a corresponding SEND block.
    Receive {
        /// Hash of the source SEND block.
        source_hash: String,
    },
    /// Revoke a DID credential.
    Revoke {
        /// Hash of the credential being revoked.
        credential_hash: String,
    },
}

/// A single block in an account chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Account public key (hex-encoded Ed25519).
    pub account: String,
    /// Hash of the previous block (empty for genesis).
    pub previous: String,
    /// The block operation type.
    pub block_type: BlockType,
    /// Account balance after this block.
    pub balance: u64,
    /// Monotonically increasing nonce (starts at 1).
    pub nonce: u64,
    /// Timestamp in milliseconds since UNIX epoch.
    pub timestamp: u64,
    /// Blake3 hash of the block contents (hex-encoded).
    pub hash: String,
    /// Ed25519 signature over raw Blake3 hash bytes.
    pub signature: Vec<u8>,
}

impl Block {
    /// Compute the Blake3 hash of block contents.
    pub fn compute_hash(
        account: &str,
        previous: &str,
        block_type: &BlockType,
        balance: u64,
        nonce: u64,
        timestamp: u64,
    ) -> String {
        let bt_json = serde_json::to_string(block_type).expect("BlockType serialization");
        let content = format!(
            "{}:{}:{}:{}:{}:{}",
            account, previous, bt_json, balance, nonce, timestamp
        );
        blake3::hash(content.as_bytes()).to_hex().to_string()
    }
}
