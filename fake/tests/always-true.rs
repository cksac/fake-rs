use rand::{rngs::mock::StepRng, Error, RngCore};
use rand_core::impls;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlwaysTrueRng {
    inner: StepRng,
    increment: u64,
}

impl AlwaysTrueRng {
    /// Implemented using StepRng's `true` range.
    /// See https://github.com/rust-random/rand/pull/1304
    /// Other "always true" rngs can be created so that they are
    /// more random, but that is not desirable for tests.
    fn new(initial: u64, increment: u64) -> Self {
        AlwaysTrueRng {
            inner: StepRng::new(initial, increment),
            increment,
        }
    }
}

impl Default for AlwaysTrueRng {
    fn default() -> Self {
        AlwaysTrueRng::new(1 << 31, (1 << 31) + 1)
    }
}

impl RngCore for AlwaysTrueRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let mut rv = self.inner.next_u64();
        if rv & (1 << 31) == 0 {
            self.inner = StepRng::new(rv | 1 << 31, self.increment);
            rv = self.inner.next_u64();
        }
        rv
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[cfg(all(test, feature = "always-true-rng"))]
mod always_true_tests {
    use super::AlwaysTrueRng;
    use fake::{Fake, Faker};

    #[test]
    fn test_rng_bool() {
        use rand::{distributions::Standard, Rng};

        // Arrange
        let rng = AlwaysTrueRng::default();

        // Act
        let result: std::vec::Vec<bool> = rng.sample_iter(Standard).take(6).collect();

        // Assert
        assert_eq!(&result, &[true, true, true, true, true, true]);
    }

    #[test]
    fn test_rng_bool_wrap_large_increment() {
        use rand::{distributions::Standard, Rng};
        let increment = 1 << 31;

        // Arrange
        let rng = AlwaysTrueRng::new(1, increment + 1);

        // Act
        let iter = rng.sample_iter(Standard).take(10000);

        // Assert
        let mut i: u64 = 0;
        iter.for_each(|x: bool| {
            assert!(x, "i = {i}");
            i = i.wrapping_add(1);
        });
    }

    #[test]
    fn test_rng_int() {
        use rand::{distributions::Standard, Rng};

        // Arrange
        let rng = AlwaysTrueRng::default();

        // Act
        let result: std::vec::Vec<u64> = rng.sample_iter(Standard).take(6).collect();

        // Assert
        assert_eq!(
            &result,
            &[
                1 << 31,
                ((1 << 31) * 3) + 1,
                ((1 << 31) * 5) + 2,
                ((1 << 31) * 7) + 3,
                ((1 << 31) * 9) + 4,
                ((1 << 31) * 11) + 5,
            ]
        );
    }

    #[test]
    fn test_rng_int_wrap() {
        use rand::{distributions::Standard, Rng};

        // Arrange
        let increment = 1 << 31;
        let rng = AlwaysTrueRng::new((increment * 2) - 2, 1);

        // Act
        let result: std::vec::Vec<u64> = rng.sample_iter(Standard).take(5).collect();

        // Assert
        assert_eq!(
            &result,
            &[
                (increment * 2) - 2,
                (increment * 2) - 1,
                increment * 3,
                (increment * 3) + 1,
                (increment * 3) + 2,
            ]
        );
    }

    #[test]
    fn test_option_never_none() {
        // Arrange
        let mut rng = AlwaysTrueRng::default();

        // Act
        let result: Option<i64> = Faker.fake_with_rng(&mut rng);

        // Assert
        assert_eq!(result, Some(6442450945));
    }

    #[test]
    fn test_array_never_empty() {
        // Arrange
        let mut rng = AlwaysTrueRng::default();

        // Act
        let result: [u8; 3] = Faker.fake_with_rng(&mut rng);

        // Assert
        assert_ne!(result.len(), 0);
    }

    #[test]
    fn test_vec_never_empty() {
        // Arrange
        let mut rng = AlwaysTrueRng::default();

        // Act
        let result: Vec<i64> = Faker.fake_with_rng(&mut rng);

        // Assert
        assert_ne!(result.len(), 0);
    }
}
