//! PN-Counter CRDT for convergent balance tracking across partitions.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Positive-Negative Counter CRDT. Commutative, associative, idempotent.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PNCounter {
    /// Positive counts (credits).
    pub p: BTreeMap<String, u64>,
    /// Negative counts (debits).
    pub n: BTreeMap<String, u64>,
}

impl PNCounter {
    /// Create a new zero counter.
    pub fn new() -> Self {
        Self {
            p: BTreeMap::new(),
            n: BTreeMap::new(),
        }
    }
    /// Increment (credit) for a node.
    pub fn increment(&mut self, node_id: &str, amount: u64) {
        let e = self.p.entry(node_id.to_string()).or_insert(0);
        *e += amount;
    }
    /// Decrement (debit) for a node.
    pub fn decrement(&mut self, node_id: &str, amount: u64) {
        let e = self.n.entry(node_id.to_string()).or_insert(0);
        *e += amount;
    }
    /// Net value (total_p - total_n).
    pub fn value(&self) -> i64 {
        let tp: u64 = self.p.values().sum();
        let tn: u64 = self.n.values().sum();
        tp as i64 - tn as i64
    }
    /// Merge with another counter (element-wise max).
    pub fn merge(&mut self, other: &PNCounter) {
        for (k, &v) in &other.p {
            let e = self.p.entry(k.clone()).or_insert(0);
            *e = (*e).max(v);
        }
        for (k, &v) in &other.n {
            let e = self.n.entry(k.clone()).or_insert(0);
            *e = (*e).max(v);
        }
    }
}

impl Default for PNCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pn_counter_increment_decrement() {
        let mut c = PNCounter::new();
        c.increment("a", 100);
        c.decrement("a", 30);
        assert_eq!(c.value(), 70);
    }

    #[test]
    fn test_pn_counter_merge() {
        let mut c1 = PNCounter::new();
        c1.increment("a", 50);
        c1.decrement("a", 10);
        let mut c2 = PNCounter::new();
        c2.increment("a", 80);
        c2.decrement("a", 5);
        c1.merge(&c2);
        assert_eq!(c1.value(), 80 - 10); // max(50,80) - max(10,5)
    }

    #[test]
    fn test_pn_counter_zero() {
        let c = PNCounter::new();
        assert_eq!(c.value(), 0);
    }

    #[test]
    fn test_pn_counter_multiple_nodes() {
        let mut c = PNCounter::new();
        c.increment("a", 100);
        c.increment("b", 200);
        c.decrement("a", 50);
        assert_eq!(c.value(), 250); // (100+200) - 50
    }
}
