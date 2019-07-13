use crate::{Dummy, Fake, Faker};
use rand::Rng;

impl<T, U> Dummy<U> for Option<T>
where
    T: Dummy<U>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &U, rng: &mut R) -> Self {
        if Faker.fake::<bool>() {
            Some(T::dummy_with_rng(config, rng))
        } else {
            None
        }
    }
}
