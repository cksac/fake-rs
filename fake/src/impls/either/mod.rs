use crate::{Dummy, Fake, Faker};
use either::Either;

impl<A, B> Dummy<Faker> for Either<A, B>
where
    A: Dummy<Faker>,
    B: Dummy<Faker>,
{
    fn dummy_with_rng<R: rand::Rng + ?Sized>(faker: &Faker, rng: &mut R) -> Self {
        if Faker.fake_with_rng(rng) {
            Either::Left(faker.fake_with_rng(rng))
        } else {
            Either::Right(faker.fake_with_rng(rng))
        }
    }
}
