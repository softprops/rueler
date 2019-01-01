#[cfg(test)]
mod tests {
    /// https://projecteuler.net/problem=1
    #[test]
    fn one() {
        assert_eq!(
            (0..1000).filter(|i| i % 3 == 0 || i % 5 == 0).sum::<u32>(),
            233_168
        )
    }

    /// https://projecteuler.net/problem=2
    #[test]
    fn two() {
        struct Fib(u64, u64);
        impl Iterator for Fib {
            type Item = u64;
            fn next(&mut self) -> Option<Self::Item> {
                let Fib(x, y) = self;
                let res = *x + *y;
                self.0 = *y;
                self.1 = res;
                Some(res)
            }
        }
        assert_eq!(
            Fib(0, 1)
                .take_while(|i| *i <= 4_000_000)
                .filter(|p| p % 2 == 0)
                .sum::<u64>(),
            4_613_732
        );
    }
}
