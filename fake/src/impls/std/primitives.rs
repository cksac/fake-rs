use crate::{Dummy, Faker};
use rand::distr::{Distribution, Uniform};
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
                rng.random()
            }
        }
    };
}

impl Dummy<usize> for usize {
    fn dummy(t: &usize) -> Self {
        *t
    }

    fn dummy_with_rng<R: Rng + ?Sized>(t: &usize, _rng: &mut R) -> Self {
        *t
    }
}

impl Dummy<Faker> for usize {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        rng.random::<u64>() as usize
    }
}

impl Dummy<isize> for isize {
    fn dummy(t: &isize) -> Self {
        *t
    }

    fn dummy_with_rng<R: Rng + ?Sized>(t: &isize, _rng: &mut R) -> Self {
        *t
    }
}

impl Dummy<Faker> for isize {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        rng.random::<i64>() as isize
    }
}

impl Dummy<Uniform<u64>> for usize {
    fn dummy_with_rng<R: Rng + ?Sized>(dist: &Uniform<u64>, rng: &mut R) -> Self {
        dist.sample(rng) as usize
    }
}

impl Dummy<Uniform<i64>> for isize {
    fn dummy_with_rng<R: Rng + ?Sized>(dist: &Uniform<i64>, rng: &mut R) -> Self {
        dist.sample(rng) as isize
    }
}

macro_rules! range_impl {
    ($typ:ident) => {
        impl Dummy<ops::Range<Self>> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::Range<Self>, rng: &mut R) -> Self {
                rng.random_range(range.start..range.end)
            }
        }

        impl Dummy<ops::RangeFrom<Self>> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeFrom<Self>, rng: &mut R) -> Self {
                let u = Uniform::new_inclusive(range.start, $typ::MAX).expect("Can sample uniform");
                u.sample(rng)
            }
        }

        impl Dummy<ops::RangeFull> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(_: &ops::RangeFull, rng: &mut R) -> Self {
                let u = Uniform::new_inclusive($typ::MIN, $typ::MAX).expect("Can sample uniform");
                u.sample(rng)
            }
        }

        impl Dummy<ops::RangeInclusive<Self>> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(
                range: &ops::RangeInclusive<Self>,
                rng: &mut R,
            ) -> Self {
                let u =
                    Uniform::new_inclusive(range.start(), range.end()).expect("Can sample uniform");
                u.sample(rng)
            }
        }

        impl Dummy<ops::RangeTo<Self>> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeTo<Self>, rng: &mut R) -> Self {
                rng.random_range($typ::MIN..range.end)
            }
        }

        impl Dummy<ops::RangeToInclusive<Self>> for $typ {
            fn dummy_with_rng<R: Rng + ?Sized>(
                range: &ops::RangeToInclusive<Self>,
                rng: &mut R,
            ) -> Self {
                let u = Uniform::new_inclusive($typ::MIN, range.end).expect("Can sample uniform");
                u.sample(rng)
            }
        }
    };
}

impl Dummy<ops::Range<Self>> for usize {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::Range<Self>, rng: &mut R) -> Self {
        rng.random_range(range.start..range.end)
    }
}

impl Dummy<ops::RangeFrom<Self>> for usize {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeFrom<Self>, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(range.start as u64, u64::MAX).expect("Can sample uniform");
        u.sample(rng) as usize
    }
}

impl Dummy<ops::RangeFull> for usize {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &ops::RangeFull, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(u64::MIN, u64::MAX).expect("Can sample uniform");
        u.sample(rng) as usize
    }
}

impl Dummy<ops::RangeInclusive<Self>> for usize {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeInclusive<Self>, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(*range.start() as u64, *range.end() as u64)
            .expect("Can sample uniform");
        u.sample(rng) as usize
    }
}

impl Dummy<ops::RangeTo<Self>> for usize {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeTo<Self>, rng: &mut R) -> Self {
        rng.random_range(u64::MIN..range.end as u64) as usize
    }
}

impl Dummy<ops::RangeToInclusive<Self>> for usize {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeToInclusive<Self>, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(u64::MIN, range.end as u64).expect("Can sample uniform");
        u.sample(rng) as usize
    }
}

impl Dummy<ops::Range<Self>> for isize {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::Range<Self>, rng: &mut R) -> Self {
        rng.random_range(range.start as i64..range.end as i64) as isize
    }
}

impl Dummy<ops::RangeFrom<Self>> for isize {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeFrom<Self>, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(range.start as i64, i64::MAX).expect("Can sample uniform");
        u.sample(rng) as isize
    }
}

impl Dummy<ops::RangeFull> for isize {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &ops::RangeFull, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(i64::MIN, i64::MAX).expect("Can sample uniform");
        u.sample(rng) as isize
    }
}

impl Dummy<ops::RangeInclusive<Self>> for isize {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeInclusive<Self>, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(*range.start() as i64, *range.end() as i64)
            .expect("Can sample uniform");
        u.sample(rng) as isize
    }
}

impl Dummy<ops::RangeTo<Self>> for isize {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeTo<Self>, rng: &mut R) -> Self {
        rng.random_range(i64::MIN..range.end as i64) as isize
    }
}

impl Dummy<ops::RangeToInclusive<Self>> for isize {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeToInclusive<Self>, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(i64::MIN, range.end as i64).expect("Can sample uniform");
        u.sample(rng) as isize
    }
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
//Manual explicit implementation for usize

integer_impl!(i8);
integer_impl!(i16);
integer_impl!(i32);
integer_impl!(i64);
#[cfg(not(target_os = "emscripten"))]
integer_impl!(i128);
//Manual explicit implementation for isize

float_impl!(f32);
float_impl!(f64);
