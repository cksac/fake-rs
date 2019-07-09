use crate::any::Any;
use crate::dummy::collections::DEFAULT_LEN_RANGE;
use crate::{Dummy, DummyAny};
use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};

impl<T, S> Dummy<Any> for HashSet<T, S>
where
    T: Dummy<Any> + Hash + Eq,
    S: BuildHasher + Default,
{
    fn dummy_ref(_: &Any) -> Self {
        let len = usize::dummy(DEFAULT_LEN_RANGE);
        let mut m = HashSet::with_capacity_and_hasher(len, S::default());
        for _ in 0..len {
            m.insert(T::any());
        }
        m
    }
}
