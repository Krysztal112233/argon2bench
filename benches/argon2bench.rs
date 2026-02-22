use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use itertools::iproduct;
use password_hash::SaltString;
use std::time::Duration;

fn bench_argon2_params(c: &mut Criterion) {
    let password = b"password";
    let salt = SaltString::generate(&mut rand::thread_rng());

    let m_values: Vec<u32> = (16384..=1048576).step_by(16384).collect();
    let t_values = [1, 2, 3];
    let p_values = [1, 2, 4, 8, 16];

    let mut group = c.benchmark_group("argon2id");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(10);

    for (m, t, p) in iproduct!(&m_values, &t_values, &p_values) {
        let params = argon2::Params::new(*m, *t, *p, None).unwrap();
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

        let bench_id = BenchmarkId::new("hash", format!("m{}_t{}_p{}", m, t, p));
        group.bench_with_input(bench_id, &(m, t, p), |b, _| {
            b.iter(|| argon2.hash_password(black_box(password), &salt).unwrap());
        });

        let hash = argon2.hash_password(black_box(password), &salt).unwrap();
        let bench_id = BenchmarkId::new("verify", format!("m{}_t{}_p{}", m, t, p));
        group.bench_with_input(bench_id, &(m, t, p), |b, _| {
            b.iter(|| argon2.verify_password(black_box(password), &hash).unwrap());
        });
    }

    group.finish();
}

criterion_group!(benches, bench_argon2_params);
criterion_main!(benches);
