use crate::{Dummy, Fake, Faker};
use bigdecimal_rs::num_bigint::{BigInt, Sign};
use rand::RngExt;

pub struct BigDecimal;
pub struct NegativeBigDecimal;
pub struct PositiveBigDecimal;
pub struct NoBigDecimalPoints;

fn create_big_decimal<R: RngExt + ?Sized>(rng: &mut R, sign: Sign) -> bigdecimal_rs::BigDecimal {
    let parts: [u32; 4] = Faker.fake_with_rng(rng);
    let int = BigInt::from_slice(sign, &parts);
    let scale = (0..64).fake_with_rng(rng);

    bigdecimal_rs::BigDecimal::new(int, scale)
}

impl Dummy<Faker> for bigdecimal_rs::BigDecimal {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let sign = if Faker.fake_with_rng(rng) {
            Sign::Plus
        } else {
            Sign::Minus
        };

        create_big_decimal(rng, sign)
    }
}

impl Dummy<BigDecimal> for bigdecimal_rs::BigDecimal {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &BigDecimal, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl Dummy<NegativeBigDecimal> for bigdecimal_rs::BigDecimal {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &NegativeBigDecimal, rng: &mut R) -> Self {
        create_big_decimal(rng, Sign::Minus)
    }
}

impl Dummy<PositiveBigDecimal> for bigdecimal_rs::BigDecimal {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &PositiveBigDecimal, rng: &mut R) -> Self {
        create_big_decimal(rng, Sign::Plus)
    }
}

impl Dummy<NoBigDecimalPoints> for bigdecimal_rs::BigDecimal {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &NoBigDecimalPoints, rng: &mut R) -> Self {
        let decimal: bigdecimal_rs::BigDecimal = Faker.fake_with_rng(rng);
        decimal.with_scale(0)
    }
}
