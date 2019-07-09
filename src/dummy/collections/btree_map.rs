use crate::any::Any;
use crate::dummy::collections::DEFAULT_LEN_RANGE;
use crate::{Dummy, DummyAny};
use std::collections::BTreeMap;

impl<K, V> Dummy<Any> for BTreeMap<K, V>
where
    K: Dummy<Any> + Ord,
    V: Dummy<Any>,
{
    fn dummy_ref(_: &Any) -> Self {
        let len = usize::dummy(DEFAULT_LEN_RANGE);
        let mut m = BTreeMap::new();
        for _ in 0..len {
            m.insert(K::any(), V::any());
        }
        m
    }
}
