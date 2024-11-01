mod common;

use common::benchmark_single_zero;
use criterion::measurement::Measurement;
use criterion::{criterion_group, criterion_main, AxisScale, Criterion, PlotConfiguration};
use std::any::type_name;
use unrolled_iterator::UnrolledIterator;

fn benchmark_every_position<T>(c: &mut Criterion)
where
    T: Copy + PartialEq + From<u8>,
{
    let mut group = c.benchmark_group(format!("position<{}>", type_name::<T>()));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    for pow in 3..16 {
        let n = 1 << pow;
        benchmark_single_zero::<T, _, _, _, _>(&mut group, n, "position", |v, f| {
            v.iter().position(f)
        });
        benchmark_single_zero::<T, _, _, _, _>(&mut group, n, "strict_position", |v, f| {
            v.iter().strict_position(f)
        });
        for k in [1, 2, 4, 8, 16] {
            benchmark_single_zero::<T, _, _, _, _>(
                &mut group,
                n,
                format!("unrolled_position(k = {})", k),
                |v, f| v.iter().unrolled_position(k, f),
            );
        }
    }
    group.finish();
}

fn benchmark_position(c: &mut Criterion) {
    benchmark_every_position::<u8>(c);
    benchmark_every_position::<u16>(c);
    benchmark_every_position::<u32>(c);
    benchmark_every_position::<u64>(c);
}

criterion_group!(benches, benchmark_position);
criterion_main!(benches);
