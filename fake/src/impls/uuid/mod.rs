use uuid::{Builder, Uuid, Variant, Version};

use crate::{Dummy, Fake, Faker};

pub struct UUIDv1;
pub struct UUIDv3;
pub struct UUIDv4;
pub struct UUIDv5;

impl Dummy<UUIDv1> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UUIDv1, rng: &mut R) -> Self {
        let ticks = Faker.fake_with_rng(rng);
        let counter = Faker.fake_with_rng(rng);
        let ts = uuid::v1::Timestamp::from_rfc4122(ticks, counter);
        let node_id: [u8; 6] = Faker.fake_with_rng(rng);
        Uuid::new_v1(ts, &node_id).expect("generate uuid::Uuid::new_v1")
    }
}

impl Dummy<UUIDv1> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UUIDv1, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng)
            .to_hyphenated()
            .to_string()
    }
}

impl Dummy<UUIDv3> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UUIDv3, rng: &mut R) -> Self {
        Builder::from_bytes(rng.gen())
            .set_variant(Variant::RFC4122)
            .set_version(Version::Md5)
            .build()
    }
}

impl Dummy<UUIDv3> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UUIDv3, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng)
            .to_hyphenated()
            .to_string()
    }
}

impl Dummy<UUIDv4> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UUIDv4, rng: &mut R) -> Self {
        Builder::from_bytes(rng.gen())
            .set_variant(Variant::RFC4122)
            .set_version(Version::Random)
            .build()
    }
}

impl Dummy<UUIDv4> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UUIDv4, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng).to_hyphenated().to_string()
    }
}

impl Dummy<UUIDv5> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UUIDv5, rng: &mut R) -> Self {
        Builder::from_bytes(rng.gen())
            .set_variant(Variant::RFC4122)
            .set_version(Version::Sha1)
            .build()
    }
}

impl Dummy<UUIDv5> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UUIDv5, rng: &mut R) -> Self {
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
