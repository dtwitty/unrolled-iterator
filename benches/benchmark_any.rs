mod common;

use common::benchmark_single_zero;
use criterion::measurement::Measurement;
use criterion::{criterion_group, criterion_main, AxisScale, Criterion, PlotConfiguration};
use std::any::type_name;
use unrolled_iterator::UnrolledIterator;

fn benchmark_every_any<T>(c: &mut Criterion)
where
    T: Copy + PartialEq + From<u8>,
{
    let mut group = c.benchmark_group(format!("any<{}>", type_name::<T>()));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    for pow in 3..16 {
        let n = 1 << pow;
        benchmark_single_zero::<T, _, _, _, _>(&mut group, n, "any", |v, f| {
            v.iter().any(f)
        });
        benchmark_single_zero::<T, _, _, _, _>(&mut group, n, "strict_any", |v, f| {
            v.iter().strict_any(f)
        });
        for k in [1, 2, 4, 8, 16] {
            benchmark_single_zero::<T, _, _, _, _>(
                &mut group,
                n,
                format!("unrolled_any(k = {})", k),
                |v, f| v.iter().unrolled_any(k, f),
            );
        }
    }
    group.finish();
}

fn benchmark_any(c: &mut Criterion) {
    benchmark_every_any::<u8>(c);
    benchmark_every_any::<u16>(c);
    benchmark_every_any::<u32>(c);
    benchmark_every_any::<u64>(c);
}

criterion_group!(benches, benchmark_any);
criterion_main!(benches);
