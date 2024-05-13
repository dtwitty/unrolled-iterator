use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, PlotConfiguration, AxisScale};
use unrolled_iterator::UnrolledIterator;

type T = u8;
fn get_random_data(n: usize) -> Vec<T>
where
{
    // Fill a vector with the numbers 1 to n inclusive.
    let mut v = vec![1 as T; n];
    // Make the last one zero.
    v[n - 1] = 0;
    v
}

fn benchmark_position(c: &mut Criterion) {
    let mut group = c.benchmark_group("position");
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    for pow in 15..20 {
        let n = 1 << pow;
        group.bench_with_input(BenchmarkId::new("position", n), &n, |b, n| {
            b.iter_batched_ref(
                || get_random_data(*n),
                |d| black_box(d.iter().position(|&x| x == 0)),
                BatchSize::SmallInput,
            )
        });

        for k in [1, 2, 4, 8, 16, 32, 64] {
            group.bench_with_input(
                BenchmarkId::new(format!("unrolled_position_{}", k), n),
                &n,
                |b, n| {
                    b.iter_batched_ref(
                        || get_random_data(*n),
                        |d| black_box(d.iter().unrolled_position(k, |&x| x == 0)),
                        BatchSize::SmallInput,
                    )
                },
            );
        }

        group.bench_with_input(BenchmarkId::new("strict_position", n), &n, |b, n| {
            b.iter_batched_ref(
                || get_random_data(*n),
                |d| black_box(d.iter().strict_position(|&x| x == 0)),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(benches, benchmark_position);
criterion_main!(benches);
