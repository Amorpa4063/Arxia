//! SLIP39 Shamir Secret Sharing for seed backup and recovery.

// TODO(M12-M18): Implement SLIP39 2-of-3 seed splitting for portable
// key recovery. Users split their Ed25519 seed into 3 shares, any 2
// of which can reconstruct the original seed.

/// Split a seed into N shares with threshold T.
pub fn split_seed(_seed: &[u8; 32], _threshold: u8, _shares: u8) -> Vec<Vec<u8>> {
    todo!("M12-M18: SLIP39 seed splitting for portable key recovery")
}

/// Reconstruct a seed from T shares.
pub fn reconstruct_seed(_shares: &[Vec<u8>]) -> Result<[u8; 32], String> {
    todo!("M12-M18: SLIP39 seed reconstruction from shares")
}
