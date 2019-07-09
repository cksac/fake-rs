use crate::{any::Any, Dummy};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::ops;

macro_rules! any {
    ($typ:ty) => {
        impl Dummy<$typ> for $typ {
            fn dummy_ref(t: &$typ) -> Self {
                t.clone()
            }
        }

        impl Dummy<Any> for $typ {
            fn dummy_ref(_: &Any) -> Self {
                rand::random()
            }
        }
    };
}

macro_rules! range {
    ($typ:ident) => {
        impl Dummy<ops::Range<Self>> for $typ {
            fn dummy_ref(range: &ops::Range<Self>) -> Self {
                rand::thread_rng().gen_range(range.start, range.end)
            }
        }

        impl Dummy<ops::RangeFrom<Self>> for $typ {
            fn dummy_ref(range: &ops::RangeFrom<Self>) -> Self {
                let u = Uniform::new_inclusive(range.start, std::$typ::MAX);
                u.sample(&mut rand::thread_rng())
            }
        }

        impl Dummy<ops::RangeFull> for $typ {
            fn dummy_ref(_: &ops::RangeFull) -> Self {
                let u = Uniform::new_inclusive(std::$typ::MIN, std::$typ::MAX);
                u.sample(&mut rand::thread_rng())
            }
        }

        impl Dummy<ops::RangeInclusive<Self>> for $typ {
            fn dummy_ref(range: &ops::RangeInclusive<Self>) -> Self {
                let u = Uniform::new_inclusive(range.start(), range.end());
                u.sample(&mut rand::thread_rng())
            }
        }

        impl Dummy<ops::RangeTo<Self>> for $typ {
            fn dummy_ref(range: &ops::RangeTo<Self>) -> Self {
                rand::thread_rng().gen_range(std::$typ::MIN, range.end)
            }
        }

        impl Dummy<ops::RangeToInclusive<Self>> for $typ {
            fn dummy_ref(range: &ops::RangeToInclusive<Self>) -> Self {
                let u = Uniform::new_inclusive(std::$typ::MIN, range.end);
                u.sample(&mut rand::thread_rng())
            }
        }
    };
}

macro_rules! number {
    ($typ:ident) => {
        impl Dummy<Uniform<Self>> for $typ {
            fn dummy_ref(dist: &Uniform<Self>) -> Self {
                dist.sample(&mut rand::thread_rng())
            }
        }
    };
}

macro_rules! integer {
    ($typ:ident) => {
        any!($typ);
        number!($typ);
        range!($typ);
    };
}

macro_rules! float {
    ($typ:ident) => {
        any!($typ);
        number!($typ);
        range!($typ);
    };
}

any!(());
any!(bool);
any!(char);

integer!(u8);
integer!(u16);
integer!(u32);
integer!(u64);
integer!(u128);
integer!(usize);

integer!(i8);
integer!(i16);
integer!(i32);
integer!(i64);
integer!(i128);
integer!(isize);

float!(f32);
float!(f64);
