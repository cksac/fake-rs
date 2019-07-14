use super::DEFAULT_LEN_RANGE;
use crate::{Dummy, Fake, Faker};
use rand::Rng;
use std::collections::HashSet;
use std::hash::{BuildHasher, Hash};

impl<T, S> Dummy<Faker> for HashSet<T, S>
where
    T: Dummy<Faker> + Hash + Eq,
    S: BuildHasher + Default,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        let len: usize = DEFAULT_LEN_RANGE.fake_with_rng(rng);
        let mut m = HashSet::with_capacity_and_hasher(len, S::default());
        for _ in 0..len {
            m.insert(config.fake_with_rng(rng));
        }
        m
    }
}
