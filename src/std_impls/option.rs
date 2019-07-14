use crate::{Dummy, Fake, Faker};
use rand::Rng;

impl<T, U> Dummy<U> for Option<T>
where
    T: Dummy<U>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &U, rng: &mut R) -> Self {
        if Faker.fake_with_rng::<bool, _>(rng) {
            Some(T::dummy_with_rng(config, rng))
        } else {
            None
        }
    }
}
