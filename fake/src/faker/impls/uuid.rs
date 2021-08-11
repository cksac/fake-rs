use uuid::Uuid;

use crate::{Dummy, Faker};
use crate::faker::uuid::raw::*;
use crate::locales::Data;

#[allow(unused_variables)]
impl<L: Data> Dummy<UuidV1<L>> for uuid::Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV1<L>, rng: &mut R) -> Self {
        todo!()
    }

    fn dummy(UuidV1(_, timestamp, node_id): &UuidV1<L>) -> Self {
        Self::new_v1(*timestamp, node_id).expect("generate uuid::Uuid::new_v1")
    }
}

#[allow(unused_variables)]
impl<L: Data> Dummy<UuidV1<L>> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV1<L>, rng: &mut R) -> Self {
        todo!()
    }

    fn dummy(config: &UuidV1<L>) -> Self {
        Uuid::dummy(config).to_hyphenated().to_string()
    }
}

#[allow(unused_variables)]
impl<L: Data> Dummy<UuidV3<L>> for uuid::Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV3<L>, rng: &mut R) -> Self {
        todo!()
    }

    fn dummy(UuidV3(_, namespace, name): &UuidV3<L>) -> Self {
        Self::new_v3(namespace, name)
    }
}

#[allow(unused_variables)]
impl<L: Data> Dummy<UuidV4<L>> for uuid::Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV4<L>, rng: &mut R) -> Self {
        todo!()
    }

    fn dummy(_: &UuidV4<L>) -> Self {
        Self::new_v4()
    }
}

#[allow(unused_variables)]
impl<L: Data> Dummy<UuidV5<L>> for uuid::Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV5<L>, rng: &mut R) -> Self {
        todo!()
    }

    fn dummy(UuidV5(_, namespace, name): &UuidV5<L>) -> Self {
        Self::new_v5(namespace, name)
    }
}

#[allow(unused_variables)]
impl<L: Data> Dummy<UuidSeed<L>> for uuid::Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidSeed<L>, rng: &mut R) -> Self {
        todo!()
    }

    fn dummy(UuidSeed(_, seed): &UuidSeed<L>) -> Self {
        Self::from_slice(&seed).expect("generate uuid::Uuid::from_slice")
    }
}

#[allow(unused_variables)]
impl Dummy<Faker> for uuid::Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        todo!()
    }

    fn dummy(config: &Faker) -> Self {
        Self::new_v4()
    }
}