use crate::{faker::boolean::en::Boolean, Dummy, Fake, Faker};
use rand::Rng;

impl<T, E> Dummy<Faker> for Result<T, E>
where
    T: Dummy<Faker>,
    E: Dummy<Faker>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        if Faker.fake_with_rng::<bool, _>(rng) {
            Ok(T::dummy_with_rng(config, rng))
        } else {
            Err(E::dummy_with_rng(config, rng))
        }
    }
}

/// Custom fake [`Result`] generator.
///
/// # Examples
///
/// ```
/// use fake::{Fake, ResultFaker};
/// use fake::faker::name::en::Name;
///
/// // generate name on success but some error code on failure
/// let f = ResultFaker::ok(Name());
/// for _ in 0..2 {
///     let a = f.fake::<Result<String, u8>>();
/// }
/// let f = ResultFaker::with(3.., 1..10);
/// for _ in 0..5 {
///     let a = f.fake::<Result<u32, usize>>();
/// }
/// ```
pub struct ResultFaker<T, E, R> {
    ok: T,
    err: E,
    err_rate: R,
}

impl Default for ResultFaker<Faker, Faker, u8> {
    fn default() -> ResultFaker<Faker, Faker, u8> {
        ResultFaker {
            ok: Faker,
            err: Faker,
            err_rate: 50,
        }
    }
}

impl<T> ResultFaker<T, Faker, u8> {
    pub fn ok(ok: T) -> ResultFaker<T, Faker, u8> {
        ResultFaker {
            ok,
            err: Faker,
            err_rate: 50,
        }
    }
}

impl<E> ResultFaker<Faker, E, u8> {
    pub fn err(err: E) -> ResultFaker<Faker, E, u8> {
        ResultFaker {
            ok: Faker,
            err,
            err_rate: 50,
        }
    }
}

impl<T, E> ResultFaker<T, E, u8> {
    pub fn with(ok: T, err: E) -> ResultFaker<T, E, u8> {
        ResultFaker {
            ok,
            err,
            err_rate: 50,
        }
    }
}

impl<T, E, R> ResultFaker<T, E, R> {
    pub fn new(ok: T, err: E, err_rate: R) -> ResultFaker<T, E, R> {
        ResultFaker { ok, err, err_rate }
    }
}

impl<T, E, U, V, X> Dummy<ResultFaker<U, V, X>> for Result<T, E>
where
    T: Dummy<U>,
    E: Dummy<V>,
    u8: Dummy<X>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &ResultFaker<U, V, X>, rng: &mut R) -> Self {
        if Boolean(config.err_rate.fake_with_rng(rng)).fake_with_rng::<bool, _>(rng) {
            Err(E::dummy_with_rng(&config.err, rng))
        } else {
            Ok(T::dummy_with_rng(&config.ok, rng))
        }
    }
}
