pub trait UnrolledIterator: ExactSizeIterator {
    fn strict_take(&mut self, n: usize) -> impl ExactSizeIterator<Item = Self::Item> {
        let len = self.len();
        if len <= n {
            self.take(len)
        } else {
            self.take(n)
        }
    }

    fn strict_any<F>(&mut self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        let mut f = f;
        self.map(&mut f).fold(false, |x, y| x | y)
    }

    fn unrolled_any<F>(&mut self, n: usize, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        let mut f = f;
        while n > 0 && self.len() >= n {
            let x = self.take(n).strict_any(&mut f);
            if x {
                return true;
            }
        }
        self.strict_any(f)
    }

    fn strict_all<F>(&mut self, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        let mut f = f;
        !self.strict_any(|x| !f(x))
    }

    fn unrolled_all<F>(&mut self, n: usize, f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        let mut f = f;
        !self.unrolled_any(n, |x| !f(x))
    }

    fn strict_position<P>(&mut self, predicate: P) -> Option<usize>
    where
        Self: Sized,
        P: FnMut(Self::Item) -> bool,
    {
        let l = self.len();
        let mut predicate = predicate;
        let min = self
            .map(&mut predicate)
            .enumerate()
            .fold(l, |x, (i, y)| x.min(if y { i } else { l }));
        (min < l).then_some(min)
    }

    fn unrolled_position<P>(&mut self, n: usize, predicate: P) -> Option<usize>
    where
        Self: Sized,
        P: FnMut(Self::Item) -> bool,
    {
        let mut predicate = predicate;

        let mut skipped = 0;
        while n > 0 && self.len() >= n {
            let min = self.take(n).strict_position(&mut predicate);
            if let Some(m) = min {
                return Some(m + skipped);
            }
            skipped += n;
        }
        self.strict_position(predicate).map(|i| i + skipped)
    }
}

impl<I: ExactSizeIterator> UnrolledIterator for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unrolled_any() {
        let vals = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        for k in 3..12 {
            for i in 1..=12 {
                let x = vals.iter().any(|&x| x < i);
                let y = vals.iter().unrolled_any(k, |&x| x < i);
                assert_eq!(x, y, "k = {}, i = {}", k, i);
            }
        }
    }

    #[test]
    fn test_unrolled_all() {
        let vals = [3, 4, 5, 6, 7, 8, 9];
        for k in 1..12 {
            for i in 1..=12 {
                let x = vals.iter().all(|&x| x < i);
                let y = vals.iter().unrolled_all(k, |&x| x < i);
                assert_eq!(x, y, "k = {}, i = {}", k, i);
            }
        }
    }

    #[test]
    fn test_position() {
        let vals = [3, 4, 5, 6, 7, 8, 9];
        for k in 1..12 {
            for i in 1..=12 {
                let x = vals.iter().position(|&x| x == i);
                let y = vals.iter().unrolled_position(k, |&x| x == i);
                assert_eq!(x, y, "k = {}, i = {}, x = {:?}, y = {:?}", k, i, x, y);
            }
        }
    }
}
