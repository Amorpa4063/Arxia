//! Fundamental types shared across the Arxia workspace.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Ed25519 public key (32 bytes).
pub type AccountId = [u8; 32];

/// Balance / amount in micro-ARX (1 ARX = 1_000_000 micro-ARX).
pub type Amount = u64;

/// Monotonically increasing nonce per account chain.
pub type Nonce = u64;

/// Blake3 hash output (32 bytes).
pub type BlockHash = [u8; 32];

/// Ed25519 signature (64 bytes).
pub type SignatureBytes = [u8; 64];

/// Block type discriminant for compact serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum BlockTypeTag {
    /// Genesis block opening an account.
    Open = 0x00,
    /// Send funds to another account.
    Send = 0x01,
    /// Receive funds from a SEND block.
    Receive = 0x02,
    /// Revoke a DID credential.
    Revoke = 0x03,
}

impl BlockTypeTag {
    /// Convert from a byte to a block type tag.
    pub fn from_byte(b: u8) -> Result<Self, crate::ArxiaError> {
        match b {
            0x00 => Ok(Self::Open),
            0x01 => Ok(Self::Send),
            0x02 => Ok(Self::Receive),
            0x03 => Ok(Self::Revoke),
            _ => Err(crate::ArxiaError::InvalidBlockType(b)),
        }
    }
}

/// Returns current time as milliseconds since UNIX epoch.
pub fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX epoch")
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_type_tag_round_trip() {
        assert_eq!(BlockTypeTag::from_byte(0x00).unwrap(), BlockTypeTag::Open);
        assert_eq!(BlockTypeTag::from_byte(0x01).unwrap(), BlockTypeTag::Send);
        assert_eq!(
            BlockTypeTag::from_byte(0x02).unwrap(),
            BlockTypeTag::Receive
        );
        assert_eq!(BlockTypeTag::from_byte(0x03).unwrap(), BlockTypeTag::Revoke);
    }

    #[test]
    fn test_block_type_tag_invalid() {
        assert!(BlockTypeTag::from_byte(0xFF).is_err());
    }

    #[test]
    fn test_now_millis_not_zero() {
        assert!(now_millis() > 0);
    }
}
