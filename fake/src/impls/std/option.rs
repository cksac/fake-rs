use crate::Dummy;
use crate::faker::boolean::en::*;
use crate::faker::option::raw::Opt;
use rand::Rng;

impl<T, U> Dummy<Opt<U>> for Option<T>
where
    T: Dummy<U>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Opt<U>, rng: &mut R) -> Self {
        if bool::dummy_with_rng(&Boolean(config.1), rng) {
            Some(T::dummy_with_rng(&config.0, rng))
        } else {
            None
        }
    }
}
