use crate::{Dummy, Fake, Faker};
use rand::Rng;

pub enum Decimal {
    Decimal,
    NegativeDecimal,
    PositiveDecimal,
    NoDecimalPoints,
}

impl Dummy<Faker> for rust_decimal::Decimal {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        rust_decimal::Decimal::from_parts(
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
            Faker.fake_with_rng(rng),
        )
    }
}

impl Dummy<Decimal> for rust_decimal::Decimal {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Decimal, rng: &mut R) -> Self {
        match config {
            Decimal::Decimal => Faker.fake(),
            Decimal::NegativeDecimal => rust_decimal::Decimal::from_parts(
                Faker.fake_with_rng(rng),
                Faker.fake_with_rng(rng),
                Faker.fake_with_rng(rng),
                true,
                Faker.fake_with_rng(rng),
            ),
            Decimal::PositiveDecimal => rust_decimal::Decimal::from_parts(
                Faker.fake_with_rng(rng),
                Faker.fake_with_rng(rng),
                Faker.fake_with_rng(rng),
                false,
                Faker.fake_with_rng(rng),
            ),
            Decimal::NoDecimalPoints => Faker.fake::<rust_decimal::Decimal>().round_dp(0),
        }
    }
}
