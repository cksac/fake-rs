use crate::faker::internet::raw::*;
use crate::locales::Data;
use crate::{Dummy, Faker};
use rand::Rng;

impl<L: Data> Dummy<Uuid<L>> for uuid::Uuid {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Uuid<L>, _: &mut R) -> Self {
        let Uuid(_, config) = config;
        match config {
            crate::faker::internet::UuidConfig::V1(timestamp, node_id) => {
                uuid::Uuid::new_v1(*timestamp, node_id).expect("generate uuid::Uuid::new_v1")
            }
            crate::faker::internet::UuidConfig::V3(namespace, name) => {
                uuid::Uuid::new_v3(namespace, name)
            }
            crate::faker::internet::UuidConfig::V5(namespace, name) => {
                uuid::Uuid::new_v5(namespace, name)
            }
            crate::faker::internet::UuidConfig::V4 => uuid::Uuid::new_v4(),
            crate::faker::internet::UuidConfig::Seed(bytes) => uuid::Uuid::from_slice(bytes).expect("generate uuid::Uuid::from_slice")
        }
    }
}

impl Dummy<Faker> for uuid::Uuid {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
        Self::new_v4()
    }
}
