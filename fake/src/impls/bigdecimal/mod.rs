use crate::decimal::*;
use crate::{Dummy, Fake, Faker};
use bigdecimal_rs as bd;
use rand::Rng;
use rust_decimal;
use std::str::FromStr;

pub struct BigDecimal;
pub struct NegativeBigDecimal;
pub struct PositiveBigDecimal;
pub struct NoBigDecimalPoints;

impl Dummy<Faker> for bd::BigDecimal {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let decimal: rust_decimal::Decimal = Faker.fake_with_rng(rng);

        bd::BigDecimal::from_str(&decimal.to_string()).unwrap()
    }
}

impl Dummy<BigDecimal> for bd::BigDecimal {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &BigDecimal, rng: &mut R) -> Self {
        let decimal: rust_decimal::Decimal = Decimal.fake_with_rng(rng);

        bd::BigDecimal::from_str(&decimal.to_string()).unwrap()
    }
}

impl Dummy<NegativeBigDecimal> for bd::BigDecimal {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &NegativeBigDecimal, rng: &mut R) -> Self {
        let decimal: rust_decimal::Decimal = NegativeDecimal.fake_with_rng(rng);

        bd::BigDecimal::from_str(&decimal.to_string()).unwrap()
    }
}

impl Dummy<PositiveBigDecimal> for bd::BigDecimal {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &PositiveBigDecimal, rng: &mut R) -> Self {
        let decimal: rust_decimal::Decimal = PositiveDecimal.fake_with_rng(rng);

        bd::BigDecimal::from_str(&decimal.to_string()).unwrap()
    }
}

impl Dummy<NoBigDecimalPoints> for bd::BigDecimal {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &NoBigDecimalPoints, rng: &mut R) -> Self {
        let decimal: rust_decimal::Decimal = NoDecimalPoints.fake_with_rng(rng);

        bd::BigDecimal::from_str(&decimal.to_string()).unwrap()
    }
}
