use uuid::{Builder, Uuid, Variant, Version};

use crate::{Dummy, Faker};
use crate::faker::uuid::raw::*;
use crate::locales::Data;

impl<L: Data> Dummy<UuidV1<L>> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV1<L>, rng: &mut R) -> Self {
      let bytes: [u8;8] = rng.gen();
      let (ticks, _) = config.1.to_rfc4122();
      let time_low = (ticks & 0xFFFF_FFFF) as u32;
      let time_mid = ((ticks >> 32) & 0xFFFF) as u16;
      let time_high_and_version = (((ticks >> 48) & 0x0FFF) as u16) | (1 << 12);
      Uuid::from_fields(time_low, time_mid, time_high_and_version, &bytes).expect("generate uuid::Uuid::from_fields")
    }

    fn dummy(UuidV1(_, timestamp, node_id): &UuidV1<L>) -> Self {
        Self::new_v1(*timestamp, node_id).expect("generate uuid::Uuid::new_v1")
    }
}

impl<L: Data> Dummy<UuidV1<L>> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV1<L>, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng).to_hyphenated().to_string()
    }

    fn dummy(config: &UuidV1<L>) -> Self {
        Uuid::dummy(config).to_hyphenated().to_string()
    }
}

impl<L: Data> Dummy<UuidV3<L>> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidV3<L>, rng: &mut R) -> Self {
        Builder::from_bytes(rng.gen())
        .set_variant(Variant::RFC4122)
        .set_version(Version::Md5)
        .build()
    }

    fn dummy(UuidV3(_, namespace, name): &UuidV3<L>) -> Self {
        Self::new_v3(namespace, name)
    }
}

impl<L: Data> Dummy<UuidV3<L>> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV3<L>, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng).to_hyphenated().to_string()
    }

    fn dummy(config: &UuidV3<L>) -> Self {
        Uuid::dummy(config).to_hyphenated().to_string()
    }
}

impl<L: Data> Dummy<UuidV4<L>> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidV4<L>, _: &mut R) -> Self {
        unimplemented!()
    }

    fn dummy(_: &UuidV4<L>) -> Self {
        Self::new_v4()
    }
}

impl<L: Data> Dummy<UuidV4<L>> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidV4<L>, _: &mut R) -> Self {
        unimplemented!()
    }

    fn dummy(config: &UuidV4<L>) -> Self {
        Uuid::dummy(config).to_hyphenated().to_string()
    }
}

impl<L: Data> Dummy<UuidV5<L>> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidV5<L>, rng: &mut R) -> Self {
      Builder::from_bytes(rng.gen())
      .set_variant(Variant::RFC4122)
      .set_version(Version::Sha1)
      .build()
    }

    fn dummy(UuidV5(_, namespace, name): &UuidV5<L>) -> Self {
        Self::new_v5(namespace, name)
    }
}

impl<L: Data> Dummy<UuidV5<L>> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV5<L>, rng: &mut R) -> Self {
        Uuid::dummy_with_rng(config, rng).to_hyphenated().to_string()
    }

    fn dummy(config: &UuidV5<L>) -> Self {
        Uuid::dummy(config).to_hyphenated().to_string()
    }
}

#[allow(unused_variables)]
impl<L: Data> Dummy<UuidSeed<L>> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidSeed<L>, rng: &mut R) -> Self {
        todo!()
        // let slice: &[u8; 128] = rng.gen();
        // Self::from_slice(slice).expect("generate uuid::Uuid::from_slice")
    }
}

#[allow(unused_variables)]
impl Dummy<Faker> for Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        todo!()
    }

    fn dummy(config: &Faker) -> Self {
        Self::new_v4()
    }
}