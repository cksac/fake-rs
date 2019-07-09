use crate::any::Any;
use crate::dummy::collections::DEFAULT_LEN_RANGE;
use crate::{Dummy, DummyAny};
use std::collections::BTreeSet;

impl<T> Dummy<Any> for BTreeSet<T>
where
    T: Dummy<Any> + Ord,
{
    fn dummy_ref(_: &Any) -> Self {
        let len = usize::dummy(DEFAULT_LEN_RANGE);
        let mut m = BTreeSet::new();
        for _ in 0..len {
            m.insert(T::any());
        }
        m
    }
}
