//! A library for generating fake data.
//!
//! # Feature flags
//!
//! - `derive`: Enable `#[derive(Dummy)]` macro
//! - `bigdecimal`: [bigdecimal](https://docs.rs/bigdecimal) integration
//! - `bson_oid`: [bson](https://docs.rs/bson) integration
//! - `chrono`: [chrono](https://docs.rs/chrono) integration
//! - `chrono-tz`: [chrono-tz](https://docs.rs/chrono-tz) integration
//! - `geo`: [geo-types](https://docs.rs/geo-types) integration
//! - `glam`: [glam](https://docs.rs/glam) integration
//! - `http`: [http](https://docs.rs/http) integration
//! - `rust-decimal`: [rust_decimal](https://docs.rs/rust_decimal) integration
//! - `time`: [time](https://docs.rs/time) integration
//! - `ulid`: [ulid](https://docs.rs/ulid) integration
//! - `uuid`: [uuid](https://docs.rs/uuid) integration
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
//! // type derived Dummy
//! let f: Foo = Faker.fake();
//! println!("{:?}", f);
//!
//! // using `Faker` to generate default fake value of given type
//! let tuple = Faker.fake::<(u8, u32, f32)>();
//! println!("tuple {:?}", tuple);
//! println!("String {:?}", Faker.fake::<String>());
//!
//! // types U can `be used to generate fake value T, if `T: Dummy<U>`
//! println!("String {:?}", (8..20).fake::<String>());
//! println!("u32 {:?}", (8..20).fake::<u32>());
//!
//! // using `faker` module with locales
//! use fake::faker::name::raw::*;
//! use fake::locales::*;
//!
//! let name: String = Name(EN).fake();
//! println!("name {:?}", name);
//!
//! let name: String = Name(ZH_TW).fake();
//! println!("name {:?}", name);
//!
//! // using convenient function without providing locale
//! use fake::faker::lorem::en::*;
//! let words: Vec<String> = Words(3..5).fake();
//! println!("words {:?}", words);
//!
//! // Using a tuple config list to generate a vector with a length range and a specific faker for the element
//! let name_vec: Vec<String> = (Name(EN), 3..5).fake();
//!
//! // Using a macro as an alternative method for the tuple config list
//! let name_vec = fake::vec![String as Name(EN); 3..5];
//!
//! // using macro to generate nested collection
//! let name_vec = fake::vec![String as Name(EN); 4, 3..5, 2];
//! println!("random nested vec {:?}", name_vec);
//!
//! // fixed seed rng
//! let seed = [
//!     1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//!     0, 0, 0, 0,
//! ];
//! let ref mut r = StdRng::from_seed(seed);
//! for _ in 0..5 {
//!     let v: usize = Faker.fake_with_rng(r);
//!     println!("value from fixed seed {}", v);
//! }
//!
//! # #[cfg(feature = "always-true-rng")] {
//! // Use an always true RNG so that optional types are always `Some` values. (Requires
//! // always-true-rng feature).
//! use fake::utils::AlwaysTrueRng;
//! let mut rng = AlwaysTrueRng::default();
//! let result: Option<i64> = Faker.fake_with_rng(&mut rng);
//! println!("Always Some: {}", result.unwrap());
//! # }
//! ```

// Enable `doc_cfg` feature for `docs.rs`
#![cfg_attr(docsrs, feature(doc_cfg))]

#[doc(hidden)]
pub use rand;
#[doc(hidden)]
pub use rand::Rng;

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
/// The trait can be used with `#[derive]` if all the type's fields
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

#[cfg(feature = "geo")]
#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
fn unique<U: Dummy<Faker> + PartialEq, R: Rng + ?Sized>(rng: &mut R, len: usize) -> Vec<U> {
    let mut set = Vec::<U>::new();
    unique_append(&mut set, rng, len);
    set
}

#[cfg(feature = "geo")]
#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
fn unique_append<U: Dummy<Faker> + PartialEq, R: Rng + ?Sized>(
    set: &mut Vec<U>,
    rng: &mut R,
    len: usize,
) {
    while set.len() != len {
        let new_item: U = Faker.fake_with_rng(rng);
        let mut found = false;
        for item in &mut *set {
            if *item == new_item {
                found = true;
                break;
            }
        }
        if !found {
            set.push(new_item);
        }
    }
}
#[macro_use]
mod impls;
pub use impls::std::option::{Opt, Optional};
pub use impls::std::path::PathFaker;
pub use impls::std::result::ResultFaker;
pub use impls::std::string::StringFaker;

#[cfg(feature = "geo")]
#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
pub use impls::geo;

#[cfg(feature = "ulid")]
#[cfg_attr(docsrs, doc(cfg(feature = "ulid")))]
pub use impls::ulid;

#[cfg(feature = "uuid")]
#[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
pub use impls::uuid;

#[cfg(feature = "rust_decimal")]
#[cfg_attr(docsrs, doc(cfg(feature = "rust_decimal")))]
pub use impls::decimal;

#[cfg(any(feature = "bigdecimal", feature = "bigdecimal-rs"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "bigdecimal", feature = "bigdecimal-rs")))
)]
pub use impls::bigdecimal;

#[cfg(feature = "serde_json")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde_json")))]
pub use impls::serde_json;

#[cfg(feature = "time")]
#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
pub use impls::time;

#[cfg(feature = "chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
pub use impls::chrono;

#[cfg(feature = "bson_oid")]
#[cfg_attr(docsrs, doc(cfg(feature = "bson_oid")))]
pub use impls::bson_oid;

/// Fake value generation for specific formats.
///
/// It is structured in a way such that the modules here describes the custom
/// group for structs implementing [`Dummy`] to generate custom fake formats.
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

/// Derive macro generating an impl of the trait [`Dummy`]. This works for both structs and enums.
///
/// # Attributes
///
/// For any fields in the type there are a number of keys that can be used to control the code generation.
/// All of these go within the dummy attribute.
///
/// 1. `faker` key can be used to provide a specific faker for a field provided it implements [`Fake`].
/// 2. `expr` key can be used to provide a rust expression as a fixed value.
/// 3. `default` key sets the value to the types [`Default`] implementation.
///
/// # Examples
///
/// A simple example for deriving [`Dummy`] on a struct:
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
///     #[dummy(expr = "\"Fixed\".into()")]
///     fixed_value: String,
///     #[dummy(default)]
///     other: String,
/// }
///
/// let f: Foo = Faker.fake();
/// ```
///
/// This would generate code roughly equivalent to:
///
/// ```
/// use fake::{Dummy, Fake, Faker};
/// use fake::faker::name::en::Name;
/// use rand::Rng;
///
/// pub struct Foo {
///     order_id: usize,
///     customer: String,
///     paid: bool,
///     fixed_value: String,
///     other: String,
/// }
///
/// impl Dummy<Faker> for Foo {
///     fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
///         let order_id = Fake::fake_with_rng::<usize, _>(&(1000..2000), rng);
///         let customer = Fake::fake_with_rng::<String, _>(&(Name()), rng);
///         let paid = Fake::fake_with_rng::<bool, _>(&Faker, rng);
///         let fixed_value = "Fixed".into();
///         let other = Default::default();
///         Self {
///             order_id,
///             customer,
///             paid,
///             fixed_value,
///             other,
///         }
///     }
/// }
///
/// let f: Foo = Faker.fake();
/// ```
///
/// A simple example for deriving [`Dummy`] on an enum. For enum tuple variants the faker attribute
/// is applied directly to the types in the tuple, for struct variants it is applied on each struct
/// field.
///
/// ```
/// use fake::{Dummy, Fake, Faker};
/// use fake::faker::name::en::Name;
///
/// #[derive(Dummy)]
/// pub enum Bar {
///     Simple,
///     Tuple(#[dummy(faker="0..5")] i32),
///     Structure {
///         #[dummy(faker = "1000..2000")]
///         i: usize,
///         j: String,
///     }
/// }
///
/// let b: Bar = Faker.fake();
/// ```
///
/// This will generate code roughly equivalent to:
///
/// ```
/// use fake::{Dummy, Fake, Faker};
/// use fake::faker::name::en::Name;
/// use rand::Rng;
///
/// pub enum Bar {
///     Simple,
///     Tuple(i32),
///     Structure {
///         i: usize,
///         j: String,
///     }
/// }
///
/// impl Dummy<Faker> for Bar {
///     fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
///         match rng.gen_range(0..3usize) {
///             0 => Self::Simple,
///             1 => Self::Tuple(Fake::fake_with_rng::<i32, _>(&(0..5), rng)),
///             2 => {
///                 let i = Fake::fake_with_rng::<usize, _>(&(1000..2000), rng);
///                 let j = Fake::fake_with_rng::<String, _>(&Faker, rng);
///                 Self::Structure { i, j }
///             },
///             _ => unreachable!(),
///         }
///     }
/// }
///
/// let b: Bar = Faker.fake();
/// ```
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use dummy::Dummy;

pub mod utils;
