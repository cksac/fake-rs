use crate::{Dummy, Fake, Faker};
use rand::Rng;
use std::collections::BinaryHeap;

impl<T> Dummy<Faker> for BinaryHeap<T>
where
    T: Dummy<Faker> + Ord,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let len = super::get_len(config, rng);
        let mut v = BinaryHeap::with_capacity(len);
        for _ in 0..len {
            v.push(config.fake_with_rng(rng));
        }
        v
    }
}

impl<T, E, L> Dummy<(E, L)> for BinaryHeap<T>
where
    T: Dummy<E> + Ord,
    usize: Dummy<L>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &(E, L), rng: &mut R) -> Self {
        let len: usize = config.1.fake_with_rng(rng);
        let mut v = BinaryHeap::with_capacity(len);
        for _ in 0..len {
            v.push(config.0.fake_with_rng(rng));
        }
        v
    }
}

#[macro_export]
macro_rules! binary_heap {
    // @ty resolve type
    (@ty $t:ty) => ($t);
    (@ty $t:ty;) => ($t);
    (@ty $t:ty; $e:expr) => (std::collections::BinaryHeap<$t>);
    (@ty $t:ty; $e:expr, $($r:tt)*) => {
        std::collections::BinaryHeap<$crate::binary_heap!(@ty $t; $($r)*)>
    };

    // @c resolve config
    (@c $e:expr; $l:expr) => (($e, $l));
    (@c $e:expr; $l:expr,) => (($e, $l));
    (@c $e:expr; $l:expr, $($r:tt)*) => {
        ($crate::binary_heap!(@c $e; $($r)*) , $l)
    };

    // use `fake::Faker` as config to generate elements
    // the element type T must be `T : Dummy<Faker>`
    ($t:ty; $($l:tt)+) => {
        $crate::binary_heap!($t as $crate::Faker; $($l)*)
    };

    // use provided config to generate elements
    ($t:ty as $e:expr; $($l:tt)+) => {
        $crate::Fake::fake::<$crate::binary_heap!(@ty $t; $($l)*)>(&$crate::binary_heap!(@c $e; $($l)*))
    };
}
