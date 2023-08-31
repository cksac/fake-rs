use crate::{Dummy, Fake};
use rand::Rng;

impl<T, U, const N: usize> Dummy<U> for [T; N]
where
    T: Dummy<U>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &U, rng: &mut R) -> Self {
        std::array::from_fn(|_| Fake::fake_with_rng::<T, _>(config, rng))
    }
}
