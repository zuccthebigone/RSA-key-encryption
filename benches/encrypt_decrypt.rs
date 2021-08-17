use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_bigint::BigUint;
use rsa::{seeded_gen_rsa_keysets, rsa_decrypt, rsa_encrypt, seeded_gen_prime};

fn encrypt_decrypt(
    public_key: impl Into<BigUint>,
    private_key: impl Into<BigUint>,
    modulus: impl Into<BigUint>,
) {
    let modulus = modulus.into();
    let msg = 12345u128;
    let ct = rsa_encrypt(msg, public_key, modulus.clone());
    black_box(rsa_decrypt(ct, private_key, modulus.clone()));
}

fn fast_prime_gen() {
    black_box(seeded_gen_prime(256));
}

fn benchmark(c: &mut Criterion) {
    let ((public_key, modulus), private_key) = seeded_gen_rsa_keysets(256);
    dbg!(&public_key, &modulus, &private_key);
    c.bench_function("encrypt_decrypt", |b| {
        b.iter(|| encrypt_decrypt(public_key.clone(), private_key.clone(), modulus.clone()))
    });
    c.bench_function("fast_prime_gen", |b| {
        b.iter(|| fast_prime_gen())
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
