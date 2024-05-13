mod common;

use criterion::measurement::Measurement;
use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkGroup, BenchmarkId, Criterion,
    PlotConfiguration,
};
use std::any::type_name;
use unrolled_iterator::UnrolledIterator;

fn benchmark_position_fn<T, M, F, O, S>(group: &mut BenchmarkGroup<M>, n: usize, name: S, mut f: F)
where
    T: Copy + PartialEq + From<u8>,
    M: Measurement,
    F: FnMut(&Vec<T>, fn(&T) -> bool) -> O,
    S: Into<String>,
{
    let input = common::zero_at_last::<T>(n);
    group.bench_with_input(BenchmarkId::new(name, n), &input, |b, v| {
        b.iter(|| f(v, |&x| x == 0.into()))
    });
}

fn benchmark_every_position<T>(c: &mut Criterion)
where
    T: Copy + PartialEq + From<u8>,
{
    let mut group = c.benchmark_group(format!("position<{}>", type_name::<T>()));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    for pow in 15..20 {
        let n = 1 << pow;
        benchmark_position_fn::<T, _, _, _, _>(&mut group, n, "position", |v, f| {
            v.iter().position(f)
        });
        benchmark_position_fn::<T, _, _, _, _>(&mut group, n, "strict_position", |v, f| {
            v.iter().strict_position(f)
        });
        for k in [1, 2, 4, 8, 16] {
            benchmark_position_fn::<T, _, _, _, _>(
                &mut group,
                n,
                format!("unrolled_position(k = {})", k),
                |v, f| {
                    v.iter().unrolled_position(k, f)
                },
            );
        }
    }
    group.finish();
}

fn benchmark_position(c: &mut Criterion) {
    benchmark_every_position::<u8 > (c);
    benchmark_every_position::<u16>(c);
    benchmark_every_position::<u32>(c);
    benchmark_every_position::<u64>(c);
}

criterion_group!(benches, benchmark_position);
criterion_main!(benches);
