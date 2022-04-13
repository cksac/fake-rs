//! A library for generating fake data.
//!
//! # Feature flags
//!
//! - `derive` provides `#[derive(Dummy)]`
//! - `chrono` [chrono](https://docs.rs/chrono) integration
//! - `http` [http](https://docs.rs/http) integration
//! - `uuid` [uuid](https://docs.rs/uuid) integration
//!
//! # Usage
//!
//! ```
//! use fake::{Dummy, Fake, Faker};
//! use rand::rngs::StdRng;
//! use rand::SeedableRng;
//!
//! #[derive(Debug, Dummy)]
//! pub struct Foo {
//!     #[dummy(faker = "1000..2000")]
//!     order_id: usize,
//!     customer: String,
//!     paid: bool,
//! }
//!
//! fn main() {
//!     // type derived Dummy
//!     let f: Foo = Faker.fake();
//!     println!("{:?}", f);
//!
//!     // using `Faker` to generate default fake value of given type
//!     let tuple = Faker.fake::<(u8, u32, f32)>();
//!     println!("tuple {:?}", tuple);
//!     println!("String {:?}", Faker.fake::<String>());
//!
//!     // types U can used to generate fake value T, if `T: Dummy<U>`
//!     println!("String {:?}", (8..20).fake::<String>());
//!     println!("u32 {:?}", (8..20).fake::<u32>());
//!
//!     // using `faker` module with locales
//!     use fake::faker::name::raw::*;
//!     use fake::locales::*;
//!
//!     let name: String = Name(EN).fake();
//!     println!("name {:?}", name);
//!
//!     let name: String = Name(ZH_TW).fake();
//!     println!("name {:?}", name);
//!
//!     // using convenient function without providing locale
//!     use fake::faker::lorem::en::*;
//!     let words: Vec<String> = Words(3..5).fake();
//!     println!("words {:?}", words);
//!
//!     // using macro to generate nested collection
//!     let name_vec = fake::vec![String as Name(EN); 4, 3..5, 2];
//!     println!("random nested vec {:?}", name_vec);
//!
//!     // fixed seed rng
//!     let seed = [
//!         1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//!         0, 0, 0, 0,
//!     ];
//!     let ref mut r = StdRng::from_seed(seed);
//!     for _ in 0..5 {
//!         let v: usize = Faker.fake_with_rng(r);
//!         println!("value from fixed seed {}", v);
//!     }
//! }
//! ```
use rand::Rng;

/// Generate default fake value for given type using [`Fake`].
///
/// # Examples
///
/// ```
/// use fake::{Fake, Faker};
///
/// let a: Option<usize> = Faker.fake();
/// // or use turbofish syntax
/// let b = Faker.fake::<Result<u32, u8>>();
/// let c: (u8, u32, f32) = Faker.fake();
/// // can also be combined with other values that implements Fake
/// let d: (u8, u32, f32) = (1..10, Faker, 2.5..5.5).fake();
/// let e: [u8; 3] = Faker.fake();
/// let f: String = Faker.fake();
/// // it also works for smart pointers and wrappers
/// let g: std::pin::Pin<String> = Faker.fake();
/// let h: Box<std::rc::Rc<u8>> = Faker.fake();
/// let i: std::path::PathBuf = Faker.fake();
/// ```
pub struct Faker;

/// Provide data structure a way to generate fake values.
/// The opposite of [`Fake`].
///
/// [`Faker`] can be used as a generic `T` for `Dummy<T>` to generate a
/// default fake value.
///
/// [`Dummy`] is similar to [`From`] trait, while [`Fake`] is similar to
/// [`Into`] trait. Except in this case [`Fake`] cannot be implemented.
///
/// # Examples
///
/// ```
/// use fake::{Dummy, Fake, Faker};
/// use rand::Rng;
/// use rand::seq::SliceRandom;
///
/// struct Name; // does not handle locale, see locales module for more
///
/// impl Dummy<Name> for &'static str {
///     fn dummy_with_rng<R: Rng + ?Sized>(_: &Name, rng: &mut R) -> &'static str {
///         const NAMES: &[&str] = &["John Doe", "Jane Doe"];
///         NAMES.choose(rng).unwrap()
///     }
/// }
///
/// let name: &str = Name.fake();
/// assert!(name == "John Doe" || name == "Jane Doe");
/// ```
///
/// # Derivable
///
/// The trait can be used with `#[derive]` if all of the type's fields
/// implement [`Fake`]. See [`Dummy`][macro@Dummy] for more.
pub trait Dummy<T>: Sized {
    /// Generate a dummy value for a type.
    ///
    /// This can be left as a blanket implemented most of the time since it
    /// uses [`Dummy::dummy_with_rng`] under the hood.
    fn dummy(config: &T) -> Self {
        let mut r = rand::thread_rng();
        Dummy::<T>::dummy_with_rng(config, &mut r)
    }

	/// Generate a dummy value for a given type using a random number
	/// generator.
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

/// Generate fake values given a type that implements [`Dummy`].
/// The opposite of [`Dummy`].
///
/// Generate default fake values with [`Faker`].
/// Generate specific fake values with helpers in [`faker`].
///
/// This trait is implemented for any type that implements [`Dummy`]:
/// [`Dummy`] should be implemented instead, and you get the [`Fake`]
/// implementation for free.
///
/// [`Dummy`] is similar to [`From`] trait, while [`Fake`] is similar to
/// [`Into`] trait. Except in this case [`Fake`] cannot be implemented.
///
/// # Examples
///
/// ```
/// use fake::Fake;
///
/// assert_eq!(10.fake::<String>().len(), 10);
/// let a: [[u8; 2]; 3] = (1..10).fake();
/// let b: Option<Option<usize>> = (1..10).fake();
/// ```
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
pub use impls::std::path::PathFaker;
pub use impls::std::result::ResultFaker;
pub use impls::std::string::StringFaker;

#[cfg(feature = "uuid")]
pub use impls::uuid;

#[cfg(feature = "rust_decimal")]
pub use impls::decimal;

/// Fake value generation for specific formats.
///
/// It is structured in a way such that the modules here describes the custom
/// group for structs implemting [`Dummy`] to generate custom fake formats.
///
/// Within the module, there is multiple modules. With `raw` module providing
/// a generic faker requiring a locale ([`faker::lorem::raw::Paragraph<L>`])
/// and the rest of the modules providing a localized faker
/// ([`faker::lorem::en::Paragraph`]) as convenient functions.
///
/// # Examples
///
/// ```
/// use fake::Fake;
/// use fake::faker::lorem::en::*;
///
/// let words: Vec<String> = Words(3..5).fake();
/// ```
pub mod faker;
/// Localized data for [`faker`]. May be incomplete.
///
/// Locales used for custom [`Dummy`] implementations within [`faker`] module.
pub mod locales;

/// Derive macro generating an impl of the trait [`Dummy`].
///
/// # Examples
///
/// ```
/// use fake::{Dummy, Fake, Faker};
/// use fake::faker::name::en::Name;
///
/// #[derive(Dummy)]
/// pub struct Foo {
///     #[dummy(faker = "1000..2000")]
///     order_id: usize,
///     #[dummy(faker = "Name()")]
///     customer: String,
///     paid: bool,
/// }
///
/// let f: Foo = Faker.fake();
/// ```
///
/// `faker` key in `dummy` attribute could be used on any type that implements
/// [`Fake`].
#[cfg(feature = "derive")]
pub use dummy::Dummy;
