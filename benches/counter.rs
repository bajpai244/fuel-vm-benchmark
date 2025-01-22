/// This benchmark is for benchmarking the performance of addition in loops in FuelVM and rEVM
/// The bytecode for both is to implement a counter, that counts from 0 to n
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_loops(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    for i in [20u64, 21u64].iter() {
        group.bench_with_input(BenchmarkId::new("Recursive", i), i, |b, i| {
            b.iter(|| fibonacci_slow(*i))
        });
        group.bench_with_input(BenchmarkId::new("Iterative", i), i, |b, i| {
            b.iter(|| fibonacci_fast(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
