use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use unrolled_iterator::UnrolledIterator;
use std::marker::Copy;

fn get_random_data<T: Default + Copy, const N: usize>() -> [T; N]
where
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    let mut data = [Default::default(); N];
    for i in 0..N {
        data[i] = Standard.sample(&mut rng);
    }
    data
}

fn benchmark_any(c: &mut Criterion) {
    let mut group = c.benchmark_group("any");
    const N: usize = 1 << 10;
    group.bench_function(BenchmarkId::new("any", N), |b| {
        b.iter_batched(
            || get_random_data::<u32, N>(),
            |d| black_box(d.iter().any(|&x| x == 0)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function(BenchmarkId::new("unrolled_any", N), |b| {
        b.iter_batched(
            || get_random_data::<u32, N>(),
            |d| black_box(d.iter().unrolled_any(32, |&x| x == 0)),
            BatchSize::SmallInput,
        )
    });

    group.bench_function(BenchmarkId::new("strict_any", N), |b| {
        b.iter_batched(
            || get_random_data::<u32, N>(),
            |d| black_box(d.iter().strict_any(|&x| x == 0)),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, benchmark_any);
criterion_main!(benches);
