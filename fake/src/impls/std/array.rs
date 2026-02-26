use crate::{Dummy, Fake};
use rand::RngExt;

impl<T, U, const N: usize> Dummy<U> for [T; N]
where
    T: Dummy<U>,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(config: &U, rng: &mut R) -> Self {
        std::array::from_fn(|_| Fake::fake_with_rng::<T, _>(config, rng))
    }
}
