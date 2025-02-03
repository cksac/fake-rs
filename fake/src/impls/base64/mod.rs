use crate::{Dummy, Fake, Faker};
use base64::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct UrlSafeBase64Value(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Base64Value(pub String);

pub struct Base64;

pub struct UrlSafeBase64;

impl Dummy<Base64> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Base64, rng: &mut R) -> Self {
        let data: Vec<u8> = Faker.fake_with_rng(rng);
        let padding = Faker.fake_with_rng(rng);
        let encoded = if padding {
            BASE64_STANDARD.encode(&data)
        } else {
            BASE64_STANDARD_NO_PAD.encode(&data)
        };
        encoded
    }
}

impl Dummy<UrlSafeBase64> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UrlSafeBase64, rng: &mut R) -> Self {
        let data: Vec<u8> = Faker.fake_with_rng(rng);
        let padding = Faker.fake_with_rng(rng);
        let encoded = if padding {
            BASE64_URL_SAFE.encode(&data)
        } else {
            BASE64_URL_SAFE_NO_PAD.encode(&data)
        };
        encoded
    }
}

impl Dummy<Faker> for Base64Value {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let s = String::dummy_with_rng(&Base64, rng);
        Base64Value(s)
    }
}

impl Dummy<Faker> for UrlSafeBase64Value {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        let s = String::dummy_with_rng(&UrlSafeBase64, rng);
        UrlSafeBase64Value(s)
    }
}
