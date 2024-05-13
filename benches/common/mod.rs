pub (crate) fn zero_at_last<T>(n: usize) -> Vec<T>
    where T: Clone + From<u8>
{
    // Fill a vector with the numbers 1 to n inclusive.
    let mut v = vec![T::from(1); n];
    // Make the last one zero.
    v[n - 1] = T::from(0);
    v
}
