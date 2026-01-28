//! Arxia full node.

use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    info!("Arxia Node v{}", env!("CARGO_PKG_VERSION"));
    info!("Initializing...");
    info!("Node ready (stub - full implementation in M6-M12)");
}
