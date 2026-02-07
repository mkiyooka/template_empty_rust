use criterion::{Criterion, criterion_group, criterion_main};
use sample::math::add;

fn benchmark_add(c: &mut Criterion) {
    c.bench_function("add 2 + 2", |b| {
        b.iter(|| add(std::hint::black_box(2), std::hint::black_box(2)))
    });
}

fn benchmark_add_large(c: &mut Criterion) {
    c.bench_function("add 1_000_000 + 1_000_000", |b| {
        b.iter(|| {
            add(
                std::hint::black_box(1_000_000),
                std::hint::black_box(1_000_000),
            )
        })
    });
}

criterion_group!(benches, benchmark_add, benchmark_add_large);
criterion_main!(benches);
