use crate::any::Any;
use crate::dummy::collections::DEFAULT_LEN_RANGE;
use crate::{Dummy, DummyAny};

impl<T> Dummy<Any> for Vec<T>
where
    T: Dummy<Any>,
{
    fn dummy_ref(_: &Any) -> Self {
        let len = usize::dummy(DEFAULT_LEN_RANGE);
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(T::any());
        }
        v
    }
}

impl<T, E, L> Dummy<(E, L)> for Vec<T>
where
    T: Dummy<E>,
    usize: Dummy<L>,
{
    fn dummy_ref(config: &(E, L)) -> Self {
        let len = usize::dummy_ref(&config.1);
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(T::dummy_ref(&config.0));
        }
        v
    }
}

#[macro_export]
macro_rules! vec {
    // @ty resolve type
    (@ty $t:ty) => ($t);
    (@ty $t:ty;) => ($t);
    (@ty $t:ty; $e:expr) => (Vec<$t>);
    (@ty $t:ty; $e:expr, $($r:tt)*) => {
        Vec<$crate::vec!(@ty $t; $($r)*)>
    };

    // @c resolve config
    (@c $e:expr; $l:expr) => (($e, $l));
    (@c $e:expr; $l:expr,) => (($e, $l));
    (@c $e:expr; $l:expr, $($r:tt)*) => {
        ($crate::vec!(@c $e; $($r)*) , $l)
    };

    // use `dummy::any::Any` as config to generate elements
    // the element type must implement `Dummy<Any>`
    ($t:ty; $($l:tt)+) => {
        $crate::vec!($t as ANY; $($l)*)
    };

    // use provided config to generate elements
    ($t:ty as $e:expr; $($l:tt)+) => {
        <$crate::vec!(@ty $t; $($l)*)>::dummy_ref(&$crate::vec!(@c $e; $($l)*))
    };
}
