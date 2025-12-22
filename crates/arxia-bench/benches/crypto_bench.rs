use criterion::{criterion_group, criterion_main, Criterion};

fn bench_blake3_hash(c: &mut Criterion) {
    let data = vec![0u8; 1024];
    c.bench_function("blake3_hash_1kb", |b| {
        b.iter(|| arxia_crypto::hash_blake3(&data))
    });
}

fn bench_ed25519_sign(c: &mut Criterion) {
    let (signing_key, _) = arxia_crypto::generate_keypair();
    let data = [0u8; 32];
    c.bench_function("ed25519_sign_32b", |b| {
        b.iter(|| arxia_crypto::sign(&signing_key, &data))
    });
}

fn bench_ed25519_verify(c: &mut Criterion) {
    let (signing_key, _) = arxia_crypto::generate_keypair();
    let data = [0u8; 32];
    let pubkey = signing_key.verifying_key().to_bytes();
    let sig = arxia_crypto::sign(&signing_key, &data);
    c.bench_function("ed25519_verify_32b", |b| {
        b.iter(|| arxia_crypto::verify(&pubkey, &data, &sig))
    });
}

criterion_group!(
    benches,
    bench_blake3_hash,
    bench_ed25519_sign,
    bench_ed25519_verify
);
criterion_main!(benches);
