//! Fake _ulid_ generation.

use ulid::Ulid;

use crate::{Dummy, Faker};

impl Dummy<Faker> for Ulid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let time_part: u64 = rng.gen_range(0..(1 << Ulid::TIME_BITS));
        let rand_part: u128 = rng.gen_range(0..(1 << Ulid::RAND_BITS));
        Ulid::from_parts(time_part, rand_part)
    }
}
