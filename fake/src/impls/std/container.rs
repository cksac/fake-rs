use crate::Dummy;
use rand::Rng;
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

macro_rules! container_impl {
    ($ptr:ident) => {
        impl<T, U> Dummy<U> for $ptr<T>
        where
            T: Dummy<U>,
        {
            fn dummy_with_rng<R: Rng + ?Sized>(config: &U, rng: &mut R) -> Self {
                $ptr::new(T::dummy_with_rng(config, rng))
            }
        }
    };
}

container_impl!(Box);
container_impl!(Cell);
container_impl!(RefCell);
container_impl!(Rc);
container_impl!(Arc);
container_impl!(Mutex);
container_impl!(RwLock);

impl<T, U> Dummy<U> for Pin<T>
where
    T: Dummy<U> + Deref,
    <T as Deref>::Target: Unpin,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &U, rng: &mut R) -> Self {
        Pin::new(T::dummy_with_rng(config, rng))
    }
}

impl<'a, T, U> Dummy<U> for Cow<'a, T>
where
    T: Dummy<U> + Clone,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &U, rng: &mut R) -> Self {
        Cow::Owned(T::dummy_with_rng(config, rng))
    }
}
