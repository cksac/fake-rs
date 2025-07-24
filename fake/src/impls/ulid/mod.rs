//! Fake _ulid_ generation.

use ulid::ULID;

use crate::{Dummy, Faker};

impl Dummy<Faker> for ULID {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let time_part: u128 = rng.random_range(0..(1 << ULID::TIMESTAMP_BITS));
        let rand_part: u128 = rng.random_range(0..(1 << ULID::RANDOM_BITS));
        ULID::from(time_part, rand_part)
    }
}
