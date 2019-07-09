use crate::any::Any;
use crate::{Dummy, DummyAny};

impl<T, E> Dummy<Any> for Result<T, E>
where
    T: Dummy<Any>,
    E: Dummy<Any>,
{
    fn dummy_ref(_: &Any) -> Self {
        if bool::any() {
            Ok(T::any())
        } else {
            Err(E::any())
        }
    }
}

impl<T, E, U, V> Dummy<(U, V)> for Result<T, E>
where
    T: Dummy<U>,
    E: Dummy<V>,
{
    fn dummy_ref(config: &(U, V)) -> Self {
        if bool::any() {
            Ok(T::dummy_ref(&config.0))
        } else {
            Err(E::dummy_ref(&config.1))
        }
    }
}
