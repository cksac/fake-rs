use crate::{Dummy, DummyAny};

impl<T, U> Dummy<U> for Option<T>
where
    T: Dummy<U>,
{
    fn dummy_ref(config: &U) -> Self {
        if bool::any() {
            Some(T::dummy_ref(config))
        } else {
            None
        }
    }
}
