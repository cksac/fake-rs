use crate::{Dummy, Fake, Faker};
use prost::Message;
use rand::Rng;

impl Dummy<Faker> for prost_types::value::Kind {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let i: u8 = (0..5).fake_with_rng(rng);
        match i {
            0 => Self::NullValue(0),
            1 => Self::NumberValue(Faker.fake_with_rng(rng)),
            2 => Self::StringValue(Faker.fake_with_rng(rng)),
            3 => Self::BoolValue(Faker.fake_with_rng(rng)),
            4 => Self::StructValue(Faker.fake_with_rng(rng)),
            5 => Self::ListValue(Faker.fake_with_rng(rng)),
            _ => unreachable!(),
        }
    }
}

impl Dummy<Faker> for prost_types::Value {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Self {
            kind: Faker.fake_with_rng(rng),
        }
    }
}

impl Dummy<Faker> for prost_types::ListValue {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Self {
            values: Faker.fake_with_rng(rng),
        }
    }
}

impl Dummy<Faker> for prost_types::Struct {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Self {
            fields: Faker.fake_with_rng(rng),
        }
    }
}

const MIN_NANOS: i32 = 0;
const MAX_NANOS: i32 = 999_999_999;

impl Dummy<Faker> for prost_types::Timestamp {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let nanos = (MIN_NANOS..MAX_NANOS).fake_with_rng(rng);
        Self {
            seconds: Faker.fake_with_rng(rng),
            nanos,
        }
    }
}

impl Dummy<Faker> for prost_types::Any {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let struct_value: prost_types::Struct = Faker.fake_with_rng(rng);

        let mut value = Vec::new();
        struct_value.encode(&mut value).unwrap();

        Self {
            type_url: "type.googleapis.com/google.protobuf.Struct".into(),
            value,
        }
    }
}
