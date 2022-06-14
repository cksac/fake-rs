//! uuid
//!
//! please note that :
//! *  all [`Dummy`] implementations for [`String`] use [to_hyphenated](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.to_hyphenated).
//! *  [`Dummy<Faker>`] implementation uses [from_u128](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.from_u128)

use uuid::{Builder, Uuid, Variant, Version};

use crate::{Dummy, Fake, Faker};

/// as per [new_v1](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v1)
pub struct UUIDv1;
/// as per [new_v3](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v3)
pub struct UUIDv3;
/// as per [new_v4](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v4)
pub struct UUIDv4;
/// as per [new_v5](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v5)
pub struct UUIDv5;

impl Dummy<UUIDv1> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UUIDv1, rng: &mut R) -> Self {
        let ticks = Faker.fake_with_rng(rng);
        let counter = Faker.fake_with_rng(rng);
        let ts = uuid::v1::Timestamp::from_rfc4122(ticks, counter);
        let node_id: [u8; 6] = Faker.fake_with_rng(rng);
        Uuid::new_v1(ts, &node_id)
    }
}

impl Dummy<UUIDv1> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UUIDv1, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng)
            .hyphenated()
            .to_string()
    }
}

impl Dummy<UUIDv3> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UUIDv3, rng: &mut R) -> Self {
        Builder::from_bytes(rng.gen())
            .with_variant(Variant::RFC4122)
            .with_version(Version::Md5)
            .into_uuid()
    }
}

impl Dummy<UUIDv3> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UUIDv3, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng)
            .hyphenated()
            .to_string()
    }
}

impl Dummy<UUIDv4> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UUIDv4, rng: &mut R) -> Self {
        Builder::from_bytes(rng.gen())
            .with_variant(Variant::RFC4122)
            .with_version(Version::Random)
            .into_uuid()
    }
}

impl Dummy<UUIDv4> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UUIDv4, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng).hyphenated().to_string()
    }
}

impl Dummy<UUIDv5> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UUIDv5, rng: &mut R) -> Self {
        Builder::from_bytes(rng.gen())
            .with_variant(Variant::RFC4122)
            .with_version(Version::Sha1)
            .into_uuid()
    }
}

impl Dummy<UUIDv5> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UUIDv5, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng)
            .hyphenated()
            .to_string()
    }
}

impl Dummy<Faker> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Self::from_u128(rng.gen())
    }
}
