//! Arxia CLI tool.

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("keygen") => cmd_keygen(),
        Some("did") => cmd_did(),
        Some("help") | Some("--help") | Some("-h") => print_help(),
        _ => print_help(),
    }
}

fn cmd_keygen() {
    let (signing_key, verifying_key) = arxia_crypto::generate_keypair();
    let public_hex = hex::encode(verifying_key.as_bytes());
    let private_hex = hex::encode(signing_key.to_bytes());

    println!("Public key:  {}", public_hex);
    println!("Private key: {}", private_hex);
    println!();
    println!("IMPORTANT: Store your private key securely.");
    println!("Never share it or commit it to version control.");
}

fn cmd_did() {
    let (_, verifying_key) = arxia_crypto::generate_keypair();
    let did = arxia_did::ArxiaDid::from_public_key(&verifying_key.to_bytes());
    println!("DID: {}", did);
}

fn print_help() {
    println!("arxia-cli - Arxia command-line interface");
    println!();
    println!("USAGE:");
    println!("  arxia-cli <COMMAND>");
    println!();
    println!("COMMANDS:");
    println!("  keygen    Generate a new Ed25519 keypair");
    println!("  did       Generate a new DID");
    println!("  help      Print this help message");
}
