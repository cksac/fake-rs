use uuid::{Builder, Uuid, Variant, Version};

use crate::{Dummy, Fake, Faker};

pub struct V1;
pub struct V3;
pub struct V4;
pub struct V5;

impl Dummy<V1> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &V1, rng: &mut R) -> Self {
        let ticks = Faker.fake_with_rng(rng);
        let counter = Faker.fake_with_rng(rng);
        let ts = uuid::v1::Timestamp::from_rfc4122(ticks, counter);
        let node_id: [u8; 6] = Faker.fake_with_rng(rng);
        Uuid::new_v1(ts, &node_id).expect("generate uuid::Uuid::new_v1")
    }
}

impl Dummy<V1> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &V1, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng)
            .to_hyphenated()
            .to_string()
    }
}

impl Dummy<V3> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &V3, rng: &mut R) -> Self {
        Builder::from_bytes(rng.gen())
            .set_variant(Variant::RFC4122)
            .set_version(Version::Md5)
            .build()
    }
}

impl Dummy<V3> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &V3, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng)
            .to_hyphenated()
            .to_string()
    }
}

impl Dummy<V4> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &V4, rng: &mut R) -> Self {
        Builder::from_bytes(rng.gen())
            .set_variant(Variant::RFC4122)
            .set_version(Version::Random)
            .build()
    }
}

impl Dummy<V4> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &V4, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng).to_hyphenated().to_string()
    }
}

impl Dummy<V5> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &V5, rng: &mut R) -> Self {
        Builder::from_bytes(rng.gen())
            .set_variant(Variant::RFC4122)
            .set_version(Version::Sha1)
            .build()
    }
}

impl Dummy<V5> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &V5, rng: &mut R) -> Self {
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
