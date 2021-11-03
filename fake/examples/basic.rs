use fake::{Dummy, Fake, Faker, ResultFaker, StringFaker};
use std::pin::Pin;
use std::rc::Rc;

#[derive(Debug, Dummy)]
pub struct Foo {
    #[dummy(faker = "1000..2000")]
    pub order_id: usize,
    pub customer: String,
    pub paid: bool,
}

fn dummy_derive() {
    let f: Foo = Faker.fake();
    println!("{:?}", f);
}

fn main() {
    dummy_derive();

    // option
    println!("Option {:?}", Faker.fake::<Option<usize>>());
    println!("Option {:?}", (1..10).fake::<Option<usize>>());
    let opt: Option<Option<usize>> = (1..10).fake();
    println!("Option {:?}", opt);

    // result
    println!("Result {:?}", Faker.fake::<Result<u32, u8>>());
    let f = ResultFaker::with(3.., 1..10);
    for _ in 0..5 {
        println!("Result {:?}", f.fake::<Result<u32, usize>>());
    }

    // tuple
    let tuple = Faker.fake::<(u8, u32, f32)>();
    println!("tuple {:?}", tuple);
    let tuple: (u8, u32, f32) = (1..10, Faker, 2.5..5.5).fake();
    println!("tuple {:?}", tuple);

    // array
    let array: [u8; 0] = Faker.fake();
    println!("array {:?}", array);
    let array: [u8; 3] = (..10).fake();
    println!("array {:?}", array);
    let array: [[u8; 2]; 3] = (1..10).fake();
    println!("nested array {:?}", array);

    // string
    println!("String {:?}", Faker.fake::<String>());
    println!("String {:?}", (8..20).fake::<String>());
    println!("Fixed length String {:?}", 10.fake::<String>());
    let f = StringFaker::with(
        String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789)(*&^%$#@!~").into_bytes(),
        8..10,
    );
    println!("String from given charset {}", f.fake::<String>());

    // containers, Box, Cell, RefCell, Rc, Arc, Mutex, RwLock, Pin
    let c: Pin<String> = Faker.fake();
    println!("Pin<String> {:?}", c);

    let c: Box<Rc<u8>> = (3..6).fake();
    println!("Box<Rc<u8>> {:?}", c);

    // path
    let path: std::path::PathBuf = Faker.fake();
    println!("PathBuf {:?}", path);
}
