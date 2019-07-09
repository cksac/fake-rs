use crate::{any::Any, Dummy, DummyAny};

macro_rules! tuple_impl {
    ($(
        $Tuple:ident {
            $(($idx:tt) $U:ident -> $T:ident)+
        }
    )+) => {
        $(
            impl<$($T:Dummy<Any>),+> Dummy<Any> for ($($T,)+) {
                #[inline]
                fn dummy_ref(_: &Any) -> ($($T,)+) {
                    ($({ let x: $T = DummyAny::any(); x},)+)
                }
            }

            impl<$($U, $T:Dummy<$U>),+> Dummy<($($U,)+)> for ($($T,)+) {
                #[inline]
                fn dummy_ref(config: &($($U,)+)) -> ($($T,)+) {
                    ($({ let x: $T = Dummy::dummy_ref(&config.$idx); x},)+)
                }
            }
        )+
    }
}

tuple_impl! {
    Tuple1 {
        (0) T0 -> A
    }
    Tuple2 {
        (0) T0 -> A
        (1) T1 -> B
    }
    Tuple3 {
        (0) T0 -> A
        (1) T1 -> B
        (2) T2 -> C
    }
    Tuple4 {
        (0) T0 -> A
        (1) T1 -> B
        (2) T2 -> C
        (3) T3 -> D
    }
    Tuple5 {
        (0) T0 -> A
        (1) T1 -> B
        (2) T2 -> C
        (3) T3 -> D
        (4) T4 -> E
    }
    Tuple6 {
        (0) T0 -> A
        (1) T1 -> B
        (2) T2 -> C
        (3) T3 -> D
        (4) T4 -> E
        (5) T5 -> F
    }
    Tuple7 {
        (0) T0 -> A
        (1) T1 -> B
        (2) T2 -> C
        (3) T3 -> D
        (4) T4 -> E
        (5) T5 -> F
        (6) T6 -> G
    }
    Tuple8 {
        (0) T0 -> A
        (1) T1 -> B
        (2) T2 -> C
        (3) T3 -> D
        (4) T4 -> E
        (5) T5 -> F
        (6) T6 -> G
        (7) T7 -> H
    }
    Tuple9 {
        (0) T0 -> A
        (1) T1 -> B
        (2) T2 -> C
        (3) T3 -> D
        (4) T4 -> E
        (5) T5 -> F
        (6) T6 -> G
        (7) T7 -> H
        (8) T8 -> I
    }
    Tuple10 {
        (0) T0 -> A
        (1) T1 -> B
        (2) T2 -> C
        (3) T3 -> D
        (4) T4 -> E
        (5) T5 -> F
        (6) T6 -> G
        (7) T7 -> H
        (8) T8 -> I
        (9) T9 -> J
    }
    Tuple11 {
        (0) T0 -> A
        (1) T1 -> B
        (2) T2 -> C
        (3) T3 -> D
        (4) T4 -> E
        (5) T5 -> F
        (6) T6 -> G
        (7) T7 -> H
        (8) T8 -> I
        (9) T9 -> J
        (10) T10 -> K
    }
    Tuple12 {
        (0) T0 -> A
        (1) T1 -> B
        (2) T2 -> C
        (3) T3 -> D
        (4) T4 -> E
        (5) T5 -> F
        (6) T6 -> G
        (7) T7 -> H
        (8) T8 -> I
        (9) T9 -> J
        (10) T10 -> K
        (11) T11 -> L
    }
}
