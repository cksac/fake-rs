use crate::{Dummy, Fake, Faker};
use rand::RngExt;
use std::collections::VecDeque;

impl<T> Dummy<Faker> for VecDeque<T>
where
    T: Dummy<Faker>,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let len = super::get_len(config, rng);
        let mut v = VecDeque::with_capacity(len);
        for _ in 0..len {
            v.push_back(config.fake_with_rng(rng));
        }
        v
    }
}

impl<T, E, L> Dummy<(E, L)> for VecDeque<T>
where
    T: Dummy<E>,
    usize: Dummy<L>,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(config: &(E, L), rng: &mut R) -> Self {
        let len: usize = config.1.fake_with_rng(rng);
        let mut v = VecDeque::with_capacity(len);
        for _ in 0..len {
            v.push_back(config.0.fake_with_rng(rng));
        }
        v
    }
}

#[macro_export]
macro_rules! vec_deque {
    // @ty resolve type
    (@ty $t:ty) => ($t);
    (@ty $t:ty;) => ($t);
    (@ty $t:ty; $e:expr) => (std::collections::VecDeque<$t>);
    (@ty $t:ty; $e:expr, $($r:tt)*) => {
        std::collections::VecDeque<$crate::vec_deque!(@ty $t; $($r)*)>
    };

    // @c resolve config
    (@c $e:expr; $l:expr) => (($e, $l));
    (@c $e:expr; $l:expr,) => (($e, $l));
    (@c $e:expr; $l:expr, $($r:tt)*) => {
        ($crate::vec_deque!(@c $e; $($r)*) , $l)
    };

    // use `fake::Faker` as config to generate elements
    // the element type T must be `T : Dummy<Faker>`
    ($t:ty; $($l:tt)+) => {
        $crate::vec_deque!($t as $crate::Faker; $($l)*)
    };

    // use provided config to generate elements
    ($t:ty as $e:expr; $($l:tt)+) => {
        $crate::Fake::fake::<$crate::vec_deque!(@ty $t; $($l)*)>(&$crate::vec_deque!(@c $e; $($l)*))
    };
}
