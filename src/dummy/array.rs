use crate::{any::Any, Dummy, DummyAny};

macro_rules! array_impl {
    {$n:expr, $t:ident $($ts:ident)*} => {
        impl<T> Dummy<[T; $n]> for [T; $n] where T: Clone {
            fn dummy_ref(t: &[T; $n]) -> Self {
                t.clone()
            }
            fn dummy(t: [T; $n]) -> Self {
                t
            }
        }

        impl<T> Dummy<Any> for [T; $n] where T: Dummy<Any> {
            fn dummy_ref(_: &Any) -> [T; $n] {
                [$t::any(), $($ts::any()),*]
            }
        }
        array_impl!{($n - 1), $($ts)*}
    };
    {$n:expr,} => {
        impl<T> Dummy<[T; $n]> for [T; $n] where T: Clone {
            fn dummy_ref(t: &[T; $n]) -> Self {
                t.clone()
            }
            fn dummy(t: [T; $n]) -> Self {
                t
            }
        }

        impl<T> Dummy<Any> for [T; $n] {
            fn dummy_ref(_: &Any) -> [T; $n] { [] }
        }
    };
}

array_impl! {32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}
