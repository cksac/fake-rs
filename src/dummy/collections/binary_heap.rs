use crate::any::Any;
use crate::dummy::collections::DEFAULT_LEN_RANGE;
use crate::{Dummy, DummyAny};
use std::collections::BinaryHeap;

impl<T> Dummy<Any> for BinaryHeap<T>
where
    T: Dummy<Any> + Ord,
{
    fn dummy_ref(_: &Any) -> Self {
        let len = usize::dummy(DEFAULT_LEN_RANGE);
        let mut v = BinaryHeap::with_capacity(len);
        for _ in 0..len {
            v.push(T::any());
        }
        v
    }
}

impl<T, E, L> Dummy<(E, L)> for BinaryHeap<T>
where
    T: Dummy<E> + Ord,
    usize: Dummy<L>,
{
    fn dummy_ref(config: &(E, L)) -> Self {
        let len = usize::dummy_ref(&config.1);
        let mut v = BinaryHeap::with_capacity(len);
        for _ in 0..len {
            v.push(T::dummy_ref(&config.0));
        }
        v
    }
}

#[macro_export]
macro_rules! binary_heap {
    // @ty resolve type
    (@ty $t:ty) => ($t);
    (@ty $t:ty;) => ($t);
    (@ty $t:ty; $e:expr) => (BinaryHeap<$t>);
    (@ty $t:ty; $e:expr, $($r:tt)*) => {
        BinaryHeap<$crate::binary_heap!(@ty $t; $($r)*)>
    };

    // @c resolve config
    (@c $e:expr; $l:expr) => (($e, $l));
    (@c $e:expr; $l:expr,) => (($e, $l));
    (@c $e:expr; $l:expr, $($r:tt)*) => {
        ($crate::binary_heap!(@c $e; $($r)*) , $l)
    };

    // use `dummy::any::Any` as config to generate elements
    // the element type must implement `Dummy<Any>`
    ($t:ty; $($l:tt)+) => {
        $crate::binary_heap!($t as ANY; $($l)*)
    };

    // use provided config to generate elements
    ($t:ty as $e:expr; $($l:tt)+) => {
        <$crate::binary_heap!(@ty $t; $($l)*)>::dummy_ref(&$crate::binary_heap!(@c $e; $($l)*))
    };
}
