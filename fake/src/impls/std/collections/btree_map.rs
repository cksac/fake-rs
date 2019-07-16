use super::DEFAULT_LEN_RANGE;
use crate::{Dummy, Fake, Faker};
use rand::Rng;
use std::collections::BTreeMap;

impl<K, V> Dummy<Faker> for BTreeMap<K, V>
where
    K: Dummy<Faker> + Ord,
    V: Dummy<Faker>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let len: usize = DEFAULT_LEN_RANGE.fake_with_rng(rng);
        let mut m = BTreeMap::new();
        for _ in 0..len {
            m.insert(config.fake_with_rng(rng), config.fake_with_rng(rng));
        }
        m
    }
}
