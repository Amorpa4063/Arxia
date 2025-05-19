//! Simulated transport for testing.
//!
//! Uses in-memory channels with configurable latency and packet loss.
//! The packet loss uses a deterministic xorshift64 PRNG for reproducibility.

use std::collections::VecDeque;

use crate::traits::{TransportError, TransportMessage, TransportTrait};

/// Deterministic xorshift64 PRNG for reproducible packet loss simulation.
fn xorshift64(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

/// A simulated transport for testing purposes.
pub struct SimulatedTransport {
    inbox: VecDeque<TransportMessage>,
    outbox: Vec<TransportMessage>,
    latency_ms: u64,
    loss_rate: f64,
    mtu: usize,
    rng_state: u64,
}

impl SimulatedTransport {
    /// Create a new simulated transport with the given parameters.
    pub fn new(latency_ms: u64, loss_rate: f64, mtu: usize) -> Self {
        Self {
            inbox: VecDeque::new(),
            outbox: Vec::new(),
            latency_ms,
            loss_rate,
            mtu,
            rng_state: 0xDEAD_BEEF_CAFE_BABE,
        }
    }

    /// Create a simulated transport with LoRa-like parameters.
    pub fn lora() -> Self {
        Self::new(2000, 0.05, 256)
    }

    /// Create a simulated transport with BLE-like parameters.
    pub fn ble() -> Self {
        Self::new(50, 0.01, 512)
    }

    /// Inject a message into the inbox (for test setup).
    pub fn inject_message(&mut self, msg: TransportMessage) {
        self.inbox.push_back(msg);
    }

    /// Get all sent messages (for test assertions).
    pub fn sent_messages(&self) -> &[TransportMessage] {
        &self.outbox
    }

    /// Get the configured latency.
    pub fn latency_ms(&self) -> u64 {
        self.latency_ms
    }
}

impl TransportTrait for SimulatedTransport {
    fn send(&mut self, msg: TransportMessage) -> Result<(), TransportError> {
        if msg.payload.len() > self.mtu {
            return Err(TransportError::PayloadTooLarge {
                size: msg.payload.len(),
                max: self.mtu,
            });
        }

        // Simulate packet loss with deterministic PRNG
        let rng_val = xorshift64(&mut self.rng_state);
        let loss_threshold = (self.loss_rate * u64::MAX as f64) as u64;
        if rng_val < loss_threshold {
            return Err(TransportError::MessageLost);
        }

        self.outbox.push(msg);
        Ok(())
    }

    fn try_recv(&mut self) -> Option<TransportMessage> {
        self.inbox.pop_front()
    }

    fn mtu(&self) -> usize {
        self.mtu
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulated_transport_send_recv() {
        let mut transport = SimulatedTransport::new(100, 0.0, 256);

        let msg = TransportMessage {
            from: "alice".to_string(),
            to: "bob".to_string(),
            payload: vec![1, 2, 3],
            timestamp: 1000,
        };

        transport.inject_message(msg.clone());
        let received = transport.try_recv();
        assert!(received.is_some());
        assert_eq!(received.unwrap().payload, vec![1, 2, 3]);
    }

    #[test]
    fn test_simulated_transport_mtu_enforcement() {
        let mut transport = SimulatedTransport::new(100, 0.0, 10);

        let msg = TransportMessage {
            from: "alice".to_string(),
            to: "bob".to_string(),
            payload: vec![0; 20],
            timestamp: 1000,
        };

        let result = transport.send(msg);
        assert!(result.is_err());
    }

    #[test]
    fn test_simulated_transport_empty_recv() {
        let mut transport = SimulatedTransport::new(100, 0.0, 256);
        assert!(transport.try_recv().is_none());
    }

    #[test]
    fn test_lora_transport_defaults() {
        let transport = SimulatedTransport::lora();
        assert_eq!(transport.mtu(), 256);
        assert_eq!(transport.latency_ms(), 2000);
    }

    #[test]
    fn test_ble_transport_defaults() {
        let transport = SimulatedTransport::ble();
        assert_eq!(transport.mtu(), 512);
        assert_eq!(transport.latency_ms(), 50);
    }

    #[test]
    fn test_xorshift64_deterministic() {
        let mut state1 = 42u64;
        let mut state2 = 42u64;
        let a = xorshift64(&mut state1);
        let b = xorshift64(&mut state2);
        assert_eq!(a, b);
    }

    #[test]
    fn test_send_captures_outbox() {
        let mut transport = SimulatedTransport::new(0, 0.0, 256);
        let msg = TransportMessage {
            from: "alice".to_string(),
            to: "bob".to_string(),
            payload: vec![42],
            timestamp: 1000,
        };
        transport.send(msg).unwrap();
        assert_eq!(transport.sent_messages().len(), 1);
        assert_eq!(transport.sent_messages()[0].payload, vec![42]);
    }
}
