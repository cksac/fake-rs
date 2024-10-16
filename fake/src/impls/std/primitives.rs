use crate::{Dummy, Faker};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::ops;

macro_rules! faker_impl {
    ($typ:ty) => {
        impl Dummy<$typ> for $typ {
            fn dummy(t: &$typ) -> Self {
                t.clone()
            }

            fn dummy_with_rng<R: Rng + ?Sized>(t: &$typ, _rng: &mut R) -> Self {
                t.clone()
            }
        }

        impl Dummy<Faker> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
                rng.gen()
            }
        }
    };
}

macro_rules! range_impl {
    ($typ:ident) => {
        impl Dummy<ops::Range<Self>> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::Range<Self>, rng: &mut R) -> Self {
                rng.gen_range(range.start..range.end)
            }
        }

        impl Dummy<ops::RangeFrom<Self>> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeFrom<Self>, rng: &mut R) -> Self {
                let u = Uniform::new_inclusive(range.start, $typ::MAX);
                u.sample(rng)
            }
        }

        impl Dummy<ops::RangeFull> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(_: &ops::RangeFull, rng: &mut R) -> Self {
                let u = Uniform::new_inclusive($typ::MIN, $typ::MAX);
                u.sample(rng)
            }
        }

        impl Dummy<ops::RangeInclusive<Self>> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(
                range: &ops::RangeInclusive<Self>,
                rng: &mut R,
            ) -> Self {
                let u = Uniform::new_inclusive(range.start(), range.end());
                u.sample(rng)
            }
        }

        impl Dummy<ops::RangeTo<Self>> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeTo<Self>, rng: &mut R) -> Self {
                rng.gen_range($typ::MIN..range.end)
            }
        }

        impl Dummy<ops::RangeToInclusive<Self>> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(
                range: &ops::RangeToInclusive<Self>,
                rng: &mut R,
            ) -> Self {
                let u = Uniform::new_inclusive($typ::MIN, range.end);
                u.sample(rng)
            }
        }
    };
}

macro_rules! number_impl {
    ($typ:ident) => {
        impl Dummy<Uniform<Self>> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(dist: &Uniform<Self>, rng: &mut R) -> Self {
                dist.sample(rng)
            }
        }
    };
}

macro_rules! integer_impl {
    ($typ:ident) => {
        faker_impl!($typ);
        number_impl!($typ);
        range_impl!($typ);
    };
}

macro_rules! float_impl {
    ($typ:ident) => {
        faker_impl!($typ);
        number_impl!($typ);
        range_impl!($typ);
    };
}

faker_impl!(());
faker_impl!(bool);
faker_impl!(char);

integer_impl!(u8);
integer_impl!(u16);
integer_impl!(u32);
integer_impl!(u64);
#[cfg(not(target_os = "emscripten"))]
integer_impl!(u128);
integer_impl!(usize);

integer_impl!(i8);
integer_impl!(i16);
integer_impl!(i32);
integer_impl!(i64);
#[cfg(not(target_os = "emscripten"))]
integer_impl!(i128);
integer_impl!(isize);

float_impl!(f32);
float_impl!(f64);
