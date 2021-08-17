use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rsa::{seeded_gen_rsa_keysets, rsa_decrypt, rsa_encrypt, seeded_gen_prime};

fn encrypt_decrypt() {
    let ((public_key, modulus), private_key) = seeded_gen_rsa_keysets(256);
    let msg = 12345u128;
    let ct = rsa_encrypt(msg, public_key, modulus.clone());
    black_box(rsa_decrypt(ct, private_key, modulus.clone()));
}

fn fast_prime_gen() {
    black_box(seeded_gen_prime(256));
}

fn rsa_keyset_gen(size: usize) {
    black_box(seeded_gen_rsa_keysets(size));
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("encrypt_decrypt", |b| {
        b.iter(|| encrypt_decrypt())
    });
    c.bench_function("fast_prime_gen", |b| {
        b.iter(|| fast_prime_gen())
    });
    c.bench_function("rsa_keyset_gen 512-bits", |b| {
        b.iter(|| rsa_keyset_gen(512))
    });
    c.bench_function("rsa_keyset_gen 1024-bits", |b| {
        b.iter(|| rsa_keyset_gen(1024))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
