use super::DEFAULT_LEN_RANGE;
use crate::{Dummy, Fake, Faker};
use rand::Rng;

impl<T> Dummy<Faker> for Vec<T>
where
    T: Dummy<Faker>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let len: usize = DEFAULT_LEN_RANGE.fake_with_rng(rng);
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(config.fake_with_rng(rng));
        }
        v
    }
}

impl<T, E, L> Dummy<(E, L)> for Vec<T>
where
    T: Dummy<E>,
    usize: Dummy<L>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &(E, L), rng: &mut R) -> Self {
        let len: usize = config.1.fake_with_rng(rng);
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(config.0.fake_with_rng(rng));
        }
        v
    }
}

/// Creates a vec with fake values.
///
/// Requires `T: Dummy<Faker>` implementation for given type.
///
/// # Examples
///
/// ```
/// let a = fake::vec![u8; 4]; // random u8 of length 4
/// let b = fake::vec![u8; 4..8]; // random u8 of length 4 to 8
/// // let c = fake::vec![u8; 4..]; // this won't work
/// ```
#[macro_export]
macro_rules! vec {
    // @ty resolve type
    (@ty $t:ty) => ($t);
    (@ty $t:ty;) => ($t);
    (@ty $t:ty; $e:expr) => (std::vec::Vec<$t>);
    (@ty $t:ty; $e:expr, $($r:tt)*) => {
        std::vec::Vec<$crate::vec!(@ty $t; $($r)*)>
    };

    // @c resolve config
    (@c $e:expr; $l:expr) => (($e, $l));
    (@c $e:expr; $l:expr,) => (($e, $l));
    (@c $e:expr; $l:expr, $($r:tt)*) => {
        ($crate::vec!(@c $e; $($r)*) , $l)
    };

    // use `fake::Faker` as config to generate elements
    // the element type T must be `T : Dummy<Faker>`
    ($t:ty; $($l:tt)+) => {
        $crate::vec!($t as $crate::Faker; $($l)*)
    };

    // use provided config to generate elements
    ($t:ty as $e:expr; $($l:tt)+) => {
        $crate::Fake::fake::<$crate::vec!(@ty $t; $($l)*)>(&$crate::vec!(@c $e; $($l)*))
    };
}
