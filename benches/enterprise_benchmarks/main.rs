use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    c.bench_function("sample_bench", |b| {
        b.iter(|| {
            // Your benchmark code here
            1 + 1
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

