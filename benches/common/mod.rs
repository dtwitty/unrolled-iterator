use crate::common;
use criterion::measurement::Measurement;
use criterion::{ BenchmarkGroup, BenchmarkId};

pub(crate) fn zero_in_middle<T>(n: usize) -> Vec<T>
where
    T: Clone + From<u8>,
{
    // Fill a vector with the numbers 1 to n inclusive.
    let mut v = vec![T::from(1); n];
    // Make the last one zero.
    v[n / 2] = T::from(0);
    v
}

pub(crate) fn benchmark_single_zero<T, M, F, O, S>(
    group: &mut BenchmarkGroup<M>,
    n: usize,
    name: S,
    mut f: F,
) where
    T: Copy + PartialEq + From<u8>,
    M: Measurement,
    F: FnMut(&Vec<T>, fn(&T) -> bool) -> O,
    S: Into<String>,
{
    let input = common::zero_in_middle::<T>(n);
    group.bench_with_input(BenchmarkId::new(name, n), &input, |b, v| {
        b.iter(|| f(v, |&x| x == 0.into()))
    });
}
