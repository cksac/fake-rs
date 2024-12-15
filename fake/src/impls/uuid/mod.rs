//! Fake _UUIDs_ generation.
//!
//! This module provides implementations for generating fake UUIDs (Universally Unique Identifiers)
//! using the [`uuid`] crate.
//!
//! Please note that:
//! * All [`Dummy`] implementations for [`String`] use [to_hyphenated](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.to_hyphenated).
//! * [`Dummy<Faker>`] implementation uses [from_u128](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.from_u128).
//!
//! # Examples
//!
//! ```rust
//! use fake::{Fake, Faker};
//! use fake::uuid::{UUIDv4, UUIDv7};
//! # use uuid::Uuid;
//!
//! // Use the `Dummy<Faker>` implementation
//! let uuid: Uuid = Faker.fake();
//!
//! // Use the `Dummy<UUIDv4>` implementation to generate a UUID instance
//! let uuid: Uuid = UUIDv4.fake();
//!
//! // Use the `Dummy<UUIDv7>` implementation to generate a UUID string
//! let uuid: String = UUIDv7.fake();
//! ```

use uuid::{Builder, Uuid, Variant, Version};

use crate::{Dummy, Fake, Faker};

/// As per [new_v1](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v1)
pub struct UUIDv1;
/// As per [new_v3](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v3)
pub struct UUIDv3;
/// As per [new_v4](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v4)
pub struct UUIDv4;
/// As per [new_v5](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v5)
pub struct UUIDv5;
/// As per [new_v6](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v6)
pub struct UUIDv6;
/// As per [new_v7](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v7)
pub struct UUIDv7;
/// As per [new_v8](https://docs.rs/uuid/latest/uuid/struct.Uuid.html#method.new_v8)
pub struct UUIDv8;

impl Dummy<UUIDv1> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UUIDv1, rng: &mut R) -> Self {
        let ticks = rng.gen_range(uuid::timestamp::UUID_TICKS_BETWEEN_EPOCHS..u64::MAX);
        let counter = Faker.fake_with_rng(rng);
        let ts = uuid::timestamp::Timestamp::from_gregorian(ticks, counter);
        let node_id: [u8; 6] = Faker.fake_with_rng(rng);
        Uuid::new_v1(ts, &node_id)
    }
}

impl Dummy<UUIDv1> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UUIDv1, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng).hyphenated().to_string()
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
        Uuid::dummy_with_rng(config, rng).hyphenated().to_string()
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
        Uuid::dummy_with_rng(config, rng).hyphenated().to_string()
    }
}

impl Dummy<UUIDv6> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UUIDv6, rng: &mut R) -> Self {
        let ticks = rng.gen_range(uuid::timestamp::UUID_TICKS_BETWEEN_EPOCHS..u64::MAX);
        let counter = Faker.fake_with_rng(rng);
        let ts = uuid::timestamp::Timestamp::from_gregorian(ticks, counter);
        let node_id: [u8; 6] = Faker.fake_with_rng(rng);
        Uuid::new_v6(ts, &node_id)
    }
}

impl Dummy<UUIDv6> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UUIDv6, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng).hyphenated().to_string()
    }
}

impl Dummy<UUIDv7> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UUIDv7, rng: &mut R) -> Self {
        let ticks = rng.gen_range(uuid::timestamp::UUID_TICKS_BETWEEN_EPOCHS..u64::MAX);
        let counter = Faker.fake_with_rng(rng);
        let ts = uuid::timestamp::Timestamp::from_gregorian(ticks, counter);
        Uuid::new_v7(ts)
    }
}

impl Dummy<UUIDv7> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UUIDv7, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng).hyphenated().to_string()
    }
}

impl Dummy<UUIDv8> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UUIDv8, rng: &mut R) -> Self {
        let buf: [u8; 16] = Faker.fake_with_rng(rng);
        Uuid::new_v8(buf)
    }
}

impl Dummy<UUIDv8> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UUIDv8, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng).hyphenated().to_string()
    }
}

impl Dummy<Faker> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Uuid::from_u128(rng.gen())
    }
}
