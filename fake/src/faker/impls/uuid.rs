use rand::Rng;

use crate::{faker::internet::raw::Uuid, impls::uuid::UuidConfig, locales::Data, Dummy};

impl<L: Data> Dummy<Uuid<L>> for uuid::Uuid {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Uuid<L>, _: &mut R) -> Self {
        match config.1 {
            UuidConfig::V1(timestamp, node_id) => {
                uuid::Uuid::new_v1(timestamp, node_id).expect("generate uuid::Uuid::new_v1")
            }
            UuidConfig::V3(namespace, name) => uuid::Uuid::new_v3(&namespace, name),
            UuidConfig::V5(namespace, name) => uuid::Uuid::new_v5(&namespace, name),
            UuidConfig::Seed(slice) => uuid::Uuid::from_u128(*slice),
            _ => uuid::Uuid::new_v4(),
        }
    }
}
