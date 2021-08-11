use uuid::{Builder, Uuid, Variant, Version};

use crate::faker::uuid::raw::*;
use crate::locales::Data;
use crate::{Dummy, Fake, Faker};

impl<L: Data> Dummy<UuidV1<L>> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidV1<L>, rng: &mut R) -> Self {
        let ticks = Faker.fake_with_rng(rng);
        let counter = Faker.fake_with_rng(rng);
        let ts = uuid::v1::Timestamp::from_rfc4122(ticks, counter);
        let node_id: [u8; 6] = Faker.fake_with_rng(rng);
        Uuid::new_v1(ts, &node_id).expect("generate uuid::Uuid::new_v1")
    }
}

impl<L: Data> Dummy<UuidV1<L>> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV1<L>, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng)
            .to_hyphenated()
            .to_string()
    }
}

impl<L: Data> Dummy<UuidV3<L>> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidV3<L>, rng: &mut R) -> Self {
        Builder::from_bytes(rng.gen())
            .set_variant(Variant::RFC4122)
            .set_version(Version::Md5)
            .build()
    }
}

impl<L: Data> Dummy<UuidV3<L>> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV3<L>, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng)
            .to_hyphenated()
            .to_string()
    }
}

impl<L: Data> Dummy<UuidV4<L>> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidV4<L>, rng: &mut R) -> Self {
        Builder::from_bytes(rng.gen())
            .set_variant(Variant::RFC4122)
            .set_version(Version::Random)
            .build()
    }
}

impl<L: Data> Dummy<UuidV4<L>> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV4<L>, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng).to_hyphenated().to_string()
    }
}

impl<L: Data> Dummy<UuidV5<L>> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidV5<L>, rng: &mut R) -> Self {
        Builder::from_bytes(rng.gen())
            .set_variant(Variant::RFC4122)
            .set_version(Version::Sha1)
            .build()
    }
}

impl<L: Data> Dummy<UuidV5<L>> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV5<L>, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng)
            .to_hyphenated()
            .to_string()
    }
}

impl Dummy<Faker> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Self::from_u128(rng.gen())
    }
}
