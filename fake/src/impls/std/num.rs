use crate::{Dummy, Faker};
use rand::distr::{Distribution, Uniform};
use rand::Rng;
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

macro_rules! signed_faker_impl {
    ($nz_typ: ty, $typ:ty) => {
        impl Dummy<Faker> for $nz_typ {
            fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
                if rng.random_bool(0.5) {
                    let u = Uniform::new_inclusive(<$typ>::MIN, -1).expect("Can sample uniform");
                    <$nz_typ>::new(u.sample(rng)).unwrap()
                } else {
                    let u = Uniform::new_inclusive(1, <$typ>::MAX).expect("Can sample uniform");
                    <$nz_typ>::new(u.sample(rng)).unwrap()
                }
            }
        }
    };
}

macro_rules! unsigned_faker_impl {
    ($nz_typ: ty, $typ:ty) => {
        impl Dummy<Faker> for $nz_typ {
            fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
                let u = Uniform::new_inclusive(1, <$typ>::MAX).expect("Can sample uniform");
                <$nz_typ>::new(u.sample(rng)).unwrap()
            }
        }
    };
}

signed_faker_impl!(NonZeroI8, i8);
signed_faker_impl!(NonZeroI16, i16);
signed_faker_impl!(NonZeroI32, i32);
signed_faker_impl!(NonZeroI64, i64);
#[cfg(not(target_os = "emscripten"))]
signed_faker_impl!(NonZeroI128, i128);

unsigned_faker_impl!(NonZeroU8, u8);
unsigned_faker_impl!(NonZeroU16, u16);
unsigned_faker_impl!(NonZeroU32, u32);
unsigned_faker_impl!(NonZeroU64, u64);
#[cfg(not(target_os = "emscripten"))]
unsigned_faker_impl!(NonZeroU128, u128);
unsigned_faker_impl!(NonZeroUsize, usize);
