use crate::{Dummy, Fake};
use rand::Rng;

macro_rules! array_impl {
    {$n:expr, $t:ident $($ts:ident)*} => {
        impl<T, U> Dummy<U> for [T; $n] where T: Dummy<U> {
            fn dummy_with_rng<R: Rng + ?Sized>(config: &U, rng: &mut R) -> Self {
                [Fake::fake_with_rng::<$t, _>(config, rng), $(Fake::fake_with_rng::<$ts, _>(config, rng)),*]
            }
        }
        array_impl!{($n - 1), $($ts)*}
    };
    {$n:expr,} => {
        impl<T, U> Dummy<U> for [T; $n] where T: Dummy<U> {
            fn dummy(_: &U) -> Self {
                []
            }

            fn dummy_with_rng<R: Rng + ?Sized>(_: &U, _rng: &mut R) -> Self {
                []
            }
        }
    };
}

array_impl! {32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}
