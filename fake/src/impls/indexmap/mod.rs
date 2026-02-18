use crate::{Dummy, Fake, Faker};
use indexmap::{IndexMap, IndexSet};
use rand::RngExt;
use std::hash::{BuildHasher, Hash};

use super::std::collections::get_len;

impl<K, V, S> Dummy<Faker> for IndexMap<K, V, S>
where
    K: Dummy<Faker> + Hash + Eq,
    V: Dummy<Faker>,
    S: BuildHasher + Default,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let len = get_len(config, rng);
        let mut m = IndexMap::with_capacity_and_hasher(len, S::default());
        for _ in 0..len {
            m.insert(config.fake_with_rng(rng), config.fake_with_rng(rng));
        }
        m
    }
}

impl<T, S> Dummy<Faker> for IndexSet<T, S>
where
    T: Dummy<Faker> + Hash + Eq,
    S: BuildHasher + Default,
{
    fn dummy_with_rng<R: RngExt + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let len = get_len(config, rng);
        let mut m = IndexSet::with_capacity_and_hasher(len, S::default());
        for _ in 0..len {
            m.insert(config.fake_with_rng(rng));
        }
        m
    }
}
