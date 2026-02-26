use crate::{faker::boolean::en::Boolean, Dummy, Fake, Faker};
use rand::RngExt;

impl<T, U> Dummy<U> for Option<T>
where
    T: Dummy<U>,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(config: &U, rng: &mut R) -> Self {
        if Faker.fake_with_rng::<bool, _>(rng) {
            Some(T::dummy_with_rng(config, rng))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Optional<T>(pub Option<T>);

impl<T> From<Optional<T>> for Option<T> {
    fn from(opt: Optional<T>) -> Self {
        opt.0
    }
}

#[derive(Debug)]
pub struct Opt<T>(pub T, pub u8);

impl<T, U> Dummy<Opt<U>> for Optional<T>
where
    T: Dummy<U>,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(config: &Opt<U>, rng: &mut R) -> Self {
        if Boolean(config.1).fake_with_rng(rng) {
            Optional(Some(config.0.fake_with_rng(rng)))
        } else {
            Optional(None)
        }
    }
}
