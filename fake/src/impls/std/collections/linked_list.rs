use crate::{Dummy, Fake, Faker};
use rand::RngExt;
use std::collections::LinkedList;

impl<T> Dummy<Faker> for LinkedList<T>
where
    T: Dummy<Faker>,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let len = super::get_len(config, rng);
        let mut v = LinkedList::new();
        for _ in 0..len {
            v.push_back(config.fake_with_rng(rng));
        }
        v
    }
}

impl<T, E, L> Dummy<(E, L)> for LinkedList<T>
where
    T: Dummy<E>,
    usize: Dummy<L>,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(config: &(E, L), rng: &mut R) -> Self {
        let len: usize = config.1.fake_with_rng(rng);
        let mut v = LinkedList::new();
        for _ in 0..len {
            v.push_back(config.0.fake_with_rng(rng));
        }
        v
    }
}

#[macro_export]
macro_rules! linked_list {
    // @ty resolve type
    (@ty $t:ty) => ($t);
    (@ty $t:ty;) => ($t);
    (@ty $t:ty; $e:expr) => (std::collections::LinkedList<$t>);
    (@ty $t:ty; $e:expr, $($r:tt)*) => {
        std::collections::LinkedList<$crate::linked_list!(@ty $t; $($r)*)>
    };

    // @c resolve config
    (@c $e:expr; $l:expr) => (($e, $l));
    (@c $e:expr; $l:expr,) => (($e, $l));
    (@c $e:expr; $l:expr, $($r:tt)*) => {
        ($crate::linked_list!(@c $e; $($r)*) , $l)
    };

    // use `fake::Faker` as config to generate elements
    // the element type T must be `T : Dummy<Faker>`
    ($t:ty; $($l:tt)+) => {
        $crate::linked_list!($t as $crate::Faker; $($l)*)
    };

    // use provided config to generate elements
    ($t:ty as $e:expr; $($l:tt)+) => {
        $crate::Fake::fake::<$crate::linked_list!(@ty $t; $($l)*)>(&$crate::linked_list!(@c $e; $($l)*))
    };
}
