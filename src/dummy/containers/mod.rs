use crate::Dummy;
use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

mod option;
mod result;

macro_rules! container {
    ($ptr:ident) => {
        impl<T, U> Dummy<U> for $ptr<T>
        where
            T: Dummy<U>,
        {
            fn dummy_ref(config: &U) -> Self {
                $ptr::new(T::dummy_ref(config))
            }
        }
    };
}

container!(Box);
container!(Cell);
container!(RefCell);
container!(Rc);
container!(Arc);
container!(Mutex);
container!(RwLock);

impl<T, U> Dummy<U> for Pin<T>
where
    T: Dummy<U> + Deref,
    <T as Deref>::Target: Unpin,
{
    fn dummy_ref(config: &U) -> Self {
        Pin::new(T::dummy_ref(config))
    }
}
