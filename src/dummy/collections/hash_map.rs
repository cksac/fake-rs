use crate::any::Any;
use crate::dummy::collections::DEFAULT_LEN_RANGE;
use crate::{Dummy, DummyAny};
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

impl<K, V, S> Dummy<Any> for HashMap<K, V, S>
where
    K: Dummy<Any> + Hash + Eq,
    V: Dummy<Any>,
    S: BuildHasher + Default,
{
    fn dummy_ref(_: &Any) -> Self {
        let len = usize::dummy(DEFAULT_LEN_RANGE);
        let mut m = HashMap::with_capacity_and_hasher(len, S::default());
        for _ in 0..len {
            m.insert(K::any(), V::any());
        }
        m
    }
}
