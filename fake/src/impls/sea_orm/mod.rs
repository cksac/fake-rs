use crate::{Dummy, Faker};
use sea_orm::prelude::Uuid;

impl Dummy<Faker> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Self::from_u128(rng.gen())
    }
}
