//! bson_objectid

use bson::oid::ObjectId;

use crate::{Dummy, Faker};

impl Dummy<Faker> for ObjectId {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, _rng: &mut R) -> Self {
        ObjectId::new()
    }
}
