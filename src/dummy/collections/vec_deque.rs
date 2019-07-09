use crate::any::Any;
use crate::dummy::collections::DEFAULT_LEN_RANGE;
use crate::{Dummy, DummyAny};
use std::collections::VecDeque;

impl<T> Dummy<Any> for VecDeque<T>
where
    T: Dummy<Any>,
{
    fn dummy_ref(_: &Any) -> Self {
        let len = usize::dummy(DEFAULT_LEN_RANGE);
        let mut v = VecDeque::with_capacity(len);
        for _ in 0..len {
            v.push_back(T::any());
        }
        v
    }
}

impl<T, E, L> Dummy<(E, L)> for VecDeque<T>
where
    T: Dummy<E>,
    usize: Dummy<L>,
{
    fn dummy_ref(config: &(E, L)) -> Self {
        let len = usize::dummy_ref(&config.1);
        let mut v = VecDeque::with_capacity(len);
        for _ in 0..len {
            v.push_back(T::dummy_ref(&config.0));
        }
        v
    }
}

#[macro_export]
macro_rules! vec_deque {
    // @ty resolve type
    (@ty $t:ty) => ($t);
    (@ty $t:ty;) => ($t);
    (@ty $t:ty; $e:expr) => (VecDeque<$t>);
    (@ty $t:ty; $e:expr, $($r:tt)*) => {
        VecDeque<$crate::vec_deque!(@ty $t; $($r)*)>
    };

    // @c resolve config
    (@c $e:expr; $l:expr) => (($e, $l));
    (@c $e:expr; $l:expr,) => (($e, $l));
    (@c $e:expr; $l:expr, $($r:tt)*) => {
        ($crate::vec_deque!(@c $e; $($r)*) , $l)
    };

    // use `dummy::any::Any` as config to generate elements
    // the element type must implement `Dummy<Any>`
    ($t:ty; $($l:tt)+) => {
        $crate::vec_deque!($t as ANY; $($l)*)
    };

    // use provided config to generate elements
    ($t:ty as $e:expr; $($l:tt)+) => {
        <$crate::vec_deque!(@ty $t; $($l)*)>::dummy_ref(&$crate::vec_deque!(@c $e; $($l)*))
    };
}
