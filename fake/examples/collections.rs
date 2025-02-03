use fake::{Fake, Faker};
use rand::distr;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

fn main() {
    let vec = fake::vec![u8; 4..8];
    println!("Vec {:?}", vec);

    let vec_deque = fake::vec_deque![u8; 4..8];
    println!("VecDeque {:?}", vec_deque);

    let linked_list = fake::linked_list![u8; 4..8];
    println!("LinkedList {:?}", linked_list);

    let binary_heap = fake::binary_heap![u8; 4..8];
    println!("BinaryHeap {:?}", binary_heap);

    let hash_map: HashMap<u8, u32> = Faker.fake();
    println!("HashMap {:?}", hash_map);

    let btree_map: BTreeMap<u8, u32> = Faker.fake();
    println!("BTreeMap {:?}", btree_map);

    let hash_set: HashSet<u32> = Faker.fake();
    println!("HashSet {:?}", hash_set);

    let btree_set: BTreeSet<u32> = Faker.fake();
    println!("BTreeSet {:?}", btree_set);

    // generate fixed length nested vec [[[u8;2];3];4] with value using sampler
    let sampler = distr::Uniform::new_inclusive(1, 10).expect("Can sample uniform");
    let v3 = fake::vec![u8 as sampler; 4, 3, 2];
    println!("random nested vec {:?}", v3);

    // generate nested type
    let v: HashMap<u8, BTreeSet<u32>> = Faker.fake();
    println!("HashMap<u8, BTreeSet<u32>> {:?}", v);
}
