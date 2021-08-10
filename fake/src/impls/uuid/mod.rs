use rand::Rng;

use crate::{Dummy, Faker};

mod v1;
mod v3;
mod v4;
mod v5;
mod seed;

impl Dummy<Faker> for uuid::Uuid {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
        Self::new_v4()
    }
}

#[derive(Debug)]
pub enum UuidConfig {
    V1(uuid::v1::Timestamp, &'static [u8]),
    V3(uuid::Uuid, &'static [u8]),
    V4,
    V5(uuid::Uuid, &'static [u8]),
    Seed(&'static u128),
}