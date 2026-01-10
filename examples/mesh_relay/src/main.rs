//! Mesh relay example.

use arxia_relay::scoring::RelayScore;
use arxia_transport::{SimulatedTransport, TransportMessage, TransportTrait};

fn main() {
    println!("=== Arxia Mesh Relay Example ===");
    println!();

    let mut transport = SimulatedTransport::lora();
    println!(
        "LoRa transport: MTU={}, latency={}ms",
        transport.mtu(),
        transport.latency_ms()
    );

    let msg = TransportMessage {
        from: "relay_node_1".to_string(),
        to: "destination".to_string(),
        payload: vec![1, 2, 3, 4, 5],
        timestamp: 1000,
    };

    match transport.send(msg) {
        Ok(()) => println!("Message sent successfully"),
        Err(e) => println!("Send failed: {}", e),
    }

    let mut score = RelayScore::new("relay_node_1".to_string());
    score.record_success();
    score.record_success();
    score.record_failure();
    println!(
        "Relay score: {} (trusted: {})",
        score.score,
        score.is_trusted()
    );

    println!();
    println!("=== Example complete ===");
}
