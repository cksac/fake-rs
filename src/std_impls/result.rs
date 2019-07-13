use crate::{Dummy, Fake, Faker};
use rand::Rng;
use typed_builder::TypedBuilder;

impl<T, E> Dummy<Faker> for Result<T, E>
where
    T: Dummy<Faker>,
    E: Dummy<Faker>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        if Faker.fake::<bool>() {
            Ok(T::dummy_with_rng(config, rng))
        } else {
            Err(E::dummy_with_rng(config, rng))
        }
    }
}

#[derive(TypedBuilder)]
pub struct ResultFaker<T, E> {
    ok: T,
    err: E,
}

impl ResultFaker<Faker, Faker> {
    pub fn default() -> ResultFaker<Faker, Faker> {
        ResultFaker {
            ok: Faker,
            err: Faker,
        }
    }
}

impl<T> ResultFaker<T, Faker> {
    pub fn ok(ok: T) -> ResultFaker<T, Faker> {
        ResultFaker { ok, err: Faker }
    }
}

impl<E> ResultFaker<Faker, E> {
    pub fn err(err: E) -> ResultFaker<Faker, E> {
        ResultFaker { ok: Faker, err }
    }
}

impl<T, E> ResultFaker<T, E> {
    pub fn with(ok: T, err: E) -> ResultFaker<T, E> {
        ResultFaker { ok, err }
    }
}

impl<T, E, U, V> Dummy<ResultFaker<U, V>> for Result<T, E>
where
    T: Dummy<U>,
    E: Dummy<V>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &ResultFaker<U, V>, rng: &mut R) -> Self {
        if Faker.fake::<bool>() {
            Ok(T::dummy_with_rng(&config.ok, rng))
        } else {
            Err(E::dummy_with_rng(&config.err, rng))
        }
    }
}
