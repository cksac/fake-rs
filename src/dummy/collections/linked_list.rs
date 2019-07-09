use crate::any::Any;
use crate::dummy::collections::DEFAULT_LEN_RANGE;
use crate::{Dummy, DummyAny};
use std::collections::LinkedList;

impl<T> Dummy<Any> for LinkedList<T>
where
    T: Dummy<Any>,
{
    fn dummy_ref(_: &Any) -> Self {
        let len = usize::dummy(DEFAULT_LEN_RANGE);
        let mut v = LinkedList::new();
        for _ in 0..len {
            v.push_back(T::any());
        }
        v
    }
}

impl<T, E, L> Dummy<(E, L)> for LinkedList<T>
where
    T: Dummy<E>,
    usize: Dummy<L>,
{
    fn dummy_ref(config: &(E, L)) -> Self {
        let len = usize::dummy_ref(&config.1);
        let mut v = LinkedList::new();
        for _ in 0..len {
            v.push_back(T::dummy_ref(&config.0));
        }
        v
    }
}

#[macro_export]
macro_rules! linked_list {
    // @ty resolve type
    (@ty $t:ty) => ($t);
    (@ty $t:ty;) => ($t);
    (@ty $t:ty; $e:expr) => (LinkedList<$t>);
    (@ty $t:ty; $e:expr, $($r:tt)*) => {
        LinkedList<$crate::linked_list!(@ty $t; $($r)*)>
    };

    // @c resolve config
    (@c $e:expr; $l:expr) => (($e, $l));
    (@c $e:expr; $l:expr,) => (($e, $l));
    (@c $e:expr; $l:expr, $($r:tt)*) => {
        ($crate::linked_list!(@c $e; $($r)*) , $l)
    };

    // use `dummy::any::Any` as config to generate elements
    // the element type must implement `Dummy<Any>`
    ($t:ty; $($l:tt)+) => {
        $crate::linked_list!($t as ANY; $($l)*)
    };

    // use provided config to generate elements
    ($t:ty as $e:expr; $($l:tt)+) => {
        <$crate::linked_list!(@ty $t; $($l)*)>::dummy_ref(&$crate::linked_list!(@c $e; $($l)*))
    };
}
