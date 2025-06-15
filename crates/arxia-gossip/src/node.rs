//! Gossip node implementation.

use crate::nonce_registry::{merge_nonce_registries, sync_nonces_before_l1, SyncResult};
use arxia_lattice::block::Block;
use std::collections::BTreeMap;

/// A gossip node that participates in the mesh network.
pub struct GossipNode {
    /// This node identifier.
    pub node_id: String,
    /// Blocks known to this node.
    pub known_blocks: Vec<Block>,
    /// Nonce registry for L1 finality tracking.
    pub nonce_registry: BTreeMap<[u8; 32], (u64, [u8; 32])>,
    /// Connected peer node IDs.
    pub peers: Vec<String>,
}

impl GossipNode {
    /// Create a new gossip node.
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            known_blocks: Vec::new(),
            nonce_registry: BTreeMap::new(),
            peers: Vec::new(),
        }
    }

    /// Add a block to the known set and register its nonce.
    pub fn add_block(&mut self, block: Block) {
        let hash_bytes: [u8; 32] = hex::decode(&block.hash)
            .unwrap_or_else(|_| vec![0u8; 32])
            .try_into()
            .unwrap_or([0u8; 32]);
        let account_bytes: [u8; 32] = hex::decode(&block.account)
            .unwrap_or_else(|_| vec![0u8; 32])
            .try_into()
            .unwrap_or([0u8; 32]);

        self.nonce_registry
            .insert(hash_bytes, (block.nonce, account_bytes));
        self.known_blocks.push(block);
    }

    /// Merge a remote nonce registry into this node registry.
    pub fn merge_registry(&mut self, remote: &BTreeMap<[u8; 32], (u64, [u8; 32])>) {
        merge_nonce_registries(&mut self.nonce_registry, remote);
    }

    /// Check sync status against a peer registry.
    pub fn check_sync(&self, peer_registry: &BTreeMap<[u8; 32], (u64, [u8; 32])>) -> SyncResult {
        sync_nonces_before_l1(&self.nonce_registry, peer_registry)
    }

    /// Add a peer.
    pub fn add_peer(&mut self, peer_id: String) {
        if !self.peers.contains(&peer_id) {
            self.peers.push(peer_id);
        }
    }
}
