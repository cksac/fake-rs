//! Fake _bson OID_ generation.

use bson::oid::ObjectId;

use crate::{Dummy, Faker};

impl Dummy<Faker> for ObjectId {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let mut bytes = [0u8; 12];
        rng.fill(&mut bytes);
        ObjectId::from_bytes(bytes)
    }
}
