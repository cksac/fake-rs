#[cfg(feature = "always-true-rng")]
mod always_true_tests {
    use fake::utils::AlwaysTrueRng;
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

    #[cfg(feature = "maybe-non-empty-collections")]
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
