use std::ops::Range;

use rand::RngExt;

use crate::{Fake, Faker};

const DEFAULT_LEN_RANGE: Range<usize> = 0..10;

pub mod binary_heap;
pub mod btree_map;
pub mod btree_set;
pub mod hash_map;
pub mod hash_set;
pub mod linked_list;
pub mod vec;
pub mod vec_deque;

#[allow(unused_mut, unused_variables)]
pub fn get_len<R: RngExt + ?Sized>(config: &Faker, rng: &mut R) -> usize {
    let mut range = DEFAULT_LEN_RANGE;
    #[cfg(feature = "maybe-non-empty-collections")]
    if config.fake_with_rng(rng) {
        // allow to use AlwaysTrueRng to generate non-empty collections
        range.start = 1;
    }
    range.fake_with_rng(rng)
}
