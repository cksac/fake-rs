use crate::{Dummy, Fake, Faker};
use rand::RngExt;
use zerocopy::{byteorder, ByteOrder};

macro_rules! byteorder_faker_impl {
    ($typ:ident) => {
        impl<O: ByteOrder> Dummy<Faker> for byteorder::$typ<O> {
            fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
                Self::new(Faker.fake_with_rng(rng))
            }
        }
    };
}

byteorder_faker_impl!(U16);
byteorder_faker_impl!(U32);
byteorder_faker_impl!(U64);
#[cfg(not(target_os = "emscripten"))]
byteorder_faker_impl!(U128);

byteorder_faker_impl!(I16);
byteorder_faker_impl!(I32);
byteorder_faker_impl!(I64);
#[cfg(not(target_os = "emscripten"))]
byteorder_faker_impl!(I128);
