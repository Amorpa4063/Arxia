//! Network partition reconciliation example.

use arxia_crdt::{CrdtVectorClock, PNCounter};

fn main() {
    println!("=== Arxia Partition Reconciliation Example ===");
    println!();

    let mut counter_a = PNCounter::new();
    let mut counter_b = PNCounter::new();

    counter_a.increment("node_1", 100);
    counter_a.increment("node_2", 50);
    counter_a.decrement("node_1", 10);

    counter_b.increment("node_3", 200);
    counter_b.decrement("node_3", 30);

    println!("Partition A value: {}", counter_a.value());
    println!("Partition B value: {}", counter_b.value());

    counter_a.merge(&counter_b);
    counter_b.merge(&counter_a);

    println!();
    println!("After merge:");
    println!("Partition A value: {}", counter_a.value());
    println!("Partition B value: {}", counter_b.value());
    assert_eq!(counter_a.value(), counter_b.value());
    println!("Values converged: OK");

    println!();
    let mut vc_a = CrdtVectorClock::new();
    let mut vc_b = CrdtVectorClock::new();
    vc_a.tick("node_1");
    vc_a.tick("node_1");
    vc_b.tick("node_2");
    println!("Concurrent: {}", vc_a.is_concurrent(&vc_b));
    vc_a.merge(&vc_b);
    println!("After merge VC_A: {:?}", vc_a);

    println!();
    println!("=== Example complete ===");
}
