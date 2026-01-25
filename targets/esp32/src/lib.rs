//! Arxia for ESP32 (no_std).
//!
//! This crate provides the ESP32 target for Arxia mesh nodes using
//! LoRa (SX1276) and BLE hardware. The 24 pre-gossip PoC v0.4.0 tests
//! run on ESP32/QEMU. Gossip tests (25-34) are x86_64 only for now.

#![no_std]
#![deny(unsafe_code)]
#![warn(missing_docs)]

// TODO(M6-M12): Gossip ESP32 port requires custom channel implementation
// over LoRa/BLE transport. std::sync::mpsc and std::time::Instant are
// unavailable in no_std. The GossipTransport trait is designed for this
// — SimulatedTransport (std) will be replaced by LoRaTransport (no_std).

// TODO(M6-M12): Implement LoRaTransport for SX1276 via embedded-hal SPI.
// TODO(M6-M12): Implement BleTransport for ESP32 BLE peripheral.
// TODO(M12-M18): Power management and deep sleep integration.

/// ESP32 Arxia module version.
pub const ESP32_VERSION: &str = "0.1.0-stub";
