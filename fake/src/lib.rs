use rand::Rng;

pub struct Faker;

pub trait Dummy<T>: Sized {
    fn dummy(config: &T) -> Self {
        let mut r = rand::thread_rng();
        Dummy::<T>::dummy_with_rng(config, &mut r)
    }

    fn dummy_with_rng<R: Rng + ?Sized>(config: &T, rng: &mut R) -> Self;
}

mod private {
    pub trait FakeBase<T>: Sized {
        fn _fake(&self) -> T;
        fn _fake_with_rng<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> T;
    }

    impl<T, U> FakeBase<U> for T
    where
        U: crate::Dummy<T>,
    {
        fn _fake(&self) -> U {
            U::dummy(self)
        }

        fn _fake_with_rng<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> U {
            U::dummy_with_rng(self, rng)
        }
    }
}

pub trait Fake: Sized {
    #[inline]
    fn fake<U>(&self) -> U
    where
        Self: private::FakeBase<U>,
    {
        self._fake()
    }

    #[inline]
    fn fake_with_rng<U, R: Rng + ?Sized>(&self, rng: &mut R) -> U
    where
        Self: private::FakeBase<U>,
    {
        self._fake_with_rng(rng)
    }
}
impl<T> Fake for T {}

#[macro_use]
mod impls;
pub use impls::std::result::ResultFaker;
pub use impls::std::string::StringFaker;

pub mod faker;
pub mod locales;

#[cfg(feature = "derive")]
pub use dummy::Dummy;
