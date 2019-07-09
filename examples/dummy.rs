use fake::*;
use fake::dummy::{ANY, distributions};
use std::collections::*;
use std::rc::Rc;

fn main() {
    // generate random u8
    println!("u8 {} in [MIN, MAX)", u8::dummy(ANY));
    println!("u8 {} in [MIN, MAX)", u8::any());

    // generate random u8 using range
    println!("u8 {} in [3,7)", u8::dummy(3..7));
    println!("u8 {} in [3,7]", u8::dummy(3..=7));
    println!("u8 {} in [3, MAX]", u8::dummy(3..));
    println!("u8 {} in [MIN, 7)", u8::dummy(..7));
    println!("u8 {} in [MIN, 7]", u8::dummy(..=7));
    println!("u8 {} in [MIN, MAX]", u8::dummy(..));

    // to reuse sampler `Uniform` for value generation
    let sampler = distributions::Uniform::new_inclusive(1, 10);
    for _ in 0..10 {
        let v = u8::dummy_ref(&sampler);
        println!("sample value {}", v);
    }

    // String
    let s = String::any();
    println!("Random String with default len range {}", s);
    let s = String::dummy("hello world");
    println!("Identity String {}", s);
    let s = String::dummy(5);
    println!("Fixed length String {}", s);
    let s = String::dummy(8..20);
    println!("Random length String {}", s);

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789)(*&^%$#@!~";
    let s = String::dummy(CHARSET);
    println!("Default random length String from given byte string {}", s);
    let s = String::dummy((CHARSET, 8));
    println!("Fixed length String from given byte string {}", s);
    let s = String::dummy((CHARSET, 8..20));
    println!("Random length String from given byte string {}", s);

    // containers, Box, Cell, RefCell, Rc, Arc, Mutex, RwLock
    let rc = <Rc<u8>>::dummy(3..6);
    println!("Rc {:?}", rc);

    let option = <Option<u32>>::any();
    println!("Option {:?}", option);

    let result = <Result<u32, u8>>::any();
    println!("Result {:?}", result);

    // generate Result type using config (R, E)
    let result = <Result<u32, u8>>::dummy((ANY, 0..2));
    println!("Result {:?}", result);

    // array
    let array = <[u8; 3]>::any();
    println!("array {:?}", array);
    let array = <[[u8; 2]; 3]>::any();
    println!("nested array {:?}", array);

    // tuple
    let tuple = <(u8, u32, f32)>::any();
    println!("tuple {:?}", tuple);
    let tuple = <(u8, u32, f32)>::dummy((sampler, ANY, 2.5..5.5));
    println!("tuple with config {:?}", tuple);

    // collections
    let vec = fake::vec![u8; 4..8];
    println!("Vec {:?}", vec);

    let vec_deque = fake::vec_deque![u8; 4..8];
    println!("VecDeque {:?}", vec_deque);

    let linked_list = fake::linked_list![u8; 4..8];
    println!("LinkedList {:?}", linked_list);

    let binary_heap = fake::binary_heap![u8; 4..8];
    println!("BinaryHeap {:?}", binary_heap);

    let hash_map = <HashMap<u8, u32>>::any();
    println!("HashMap {:?}", hash_map);

    let btree_map = <BTreeMap<u8, u32>>::any();
    println!("BTreeMap {:?}", btree_map);

    let hash_set = <HashSet<u32>>::any();
    println!("HashSet {:?}", hash_set);

    let btree_set = <BTreeSet<u32>>::any();
    println!("BTreeSet {:?}", btree_set);

    // generate vec with default random length [0..10)
    let v1 = <Vec<u8>>::any();
    println!("default random length vec {:?}", v1);

    // generate random Vec<u8> with fixed length
    let v1 = <Vec<u8>>::dummy((ANY, 5));
    let v2 = fake::vec![u8; 5];
    println!("fixed length vec {:?}", v1);
    println!("fixed length vec {:?}", v2);

    // generate random Vec<u8> with random length
    let v1 = <Vec<u8>>::dummy((ANY, 2..5));
    let v2 = fake::vec![u8; 2..5];
    println!("random length vec {:?}", v1);
    println!("random length vec {:?}", v2);

    // generate random Vec<u8> with random length and value config
    let v1 = <Vec<u8>>::dummy((1..=10, 2..4));
    let v2 = fake::vec![u8 as 1..=10; 2..4];
    println!("fixed length and element config vec {:?}", v1);
    println!("fixed length and element config vec {:?}", v2);

    // generate nested vec
    let v1 = <Vec<Vec<u8>>>::dummy(((ANY, 1..3), 5));
    let v2 = fake::vec![u8; 5, 1..3];
    println!("random nested vec {:?}", v1);
    println!("random nested vec {:?}", v2);

    // generate fixed length nested vec [[[u8;2];3];4] with value using sampler
    let v3 = fake::vec![u8 as sampler; 4, 3, 2];
    println!("random nested vec {:?}", v3);

    // generate nested type and use `DummyAny::any`
    let v: HashMap<u8, BTreeSet<u32>> = DummyAny::any();
    println!("HashMap<u8, BTreeSet<u32>> {:?}", v);
}
