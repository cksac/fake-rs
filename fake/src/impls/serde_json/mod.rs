use std::collections::HashMap;
use std::ops::Range;

use serde_json::{Map, Number, Value};

use crate::{Dummy, Fake, Faker};

const JSON_UNION_MEMBERS_NON_RECURSIVE: Range<usize> = 0..4;
const JSON_UNION_MEMBERS: Range<usize> = 0..6;

// The Mapping cant include arrays or mappings
// to avoid overflowing the stack.
struct NonRecursiveValue(Value);

impl Dummy<Faker> for NonRecursiveValue {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let union_index: usize = JSON_UNION_MEMBERS_NON_RECURSIVE.fake_with_rng(rng);
        match union_index {
            0 => NonRecursiveValue(serde_json::Value::Null),
            1 => NonRecursiveValue(serde_json::Value::Bool(config.fake_with_rng(rng))),
            2 => NonRecursiveValue(serde_json::Value::Number(config.fake_with_rng(rng))),
            3 => NonRecursiveValue(serde_json::Value::String(config.fake_with_rng(rng))),
            _ => panic!(),
        }
    }
}

impl Dummy<Faker> for Value {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let union_index: usize = JSON_UNION_MEMBERS.fake_with_rng(rng);
        match union_index {
            0 => serde_json::Value::Null,
            1 => serde_json::Value::Bool(config.fake_with_rng(rng)),
            2 => serde_json::Value::Number(config.fake_with_rng(rng)),
            3 => serde_json::Value::String(config.fake_with_rng(rng)),
            4 => serde_json::Value::Array(config.fake_with_rng(rng)),
            5 => serde_json::Value::Object(config.fake_with_rng(rng)),
            _ => panic!(),
        }
    }
}

impl Dummy<Faker> for Number {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        // Alternate between generating floats and integers
        if Faker.fake_with_rng::<bool, _>(rng) {
            Number::from_f64(config.fake_with_rng::<f64, _>(rng)).unwrap()
        } else {
            Number::from_f64(config.fake_with_rng::<i32, _>(rng).into()).unwrap()
        }
    }
}

impl Dummy<Faker> for Map<String, Value> {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        Map::from_iter(
            config
                .fake_with_rng::<HashMap<String, NonRecursiveValue>, _>(rng)
                .iter()
                .map(|(x, y)| (x.to_owned(), y.0.to_owned())),
        )
    }
}
