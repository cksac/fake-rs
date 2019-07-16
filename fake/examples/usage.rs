use fake::{Dummy, Fake, Faker};
use rand::rngs::StdRng;
use rand::SeedableRng;

#[derive(Debug, Dummy)]
pub struct Foo {
    #[dummy(faker = "1000..2000")]
    order_id: usize,
    customer: String,
    paid: bool,
}

fn main() {
    // type derived Dummy
    let f: Foo = Faker.fake();
    println!("{:?}", f);

    // using `Faker` to generate default fake value of given type
    let tuple = Faker.fake::<(u8, u32, f32)>();
    println!("tuple {:?}", tuple);
    println!("String {:?}", Faker.fake::<String>());

    // types U can used to generate fake value T, if `T: Dummy<U>`
    println!("String {:?}", (8..20).fake::<String>());
    println!("u32 {:?}", (8..20).fake::<u32>());

    // using `faker` module with locales
    use fake::faker::name::raw::*;
    use fake::locales::*;

    let name: String = Name(EN).fake();
    println!("name {:?}", name);

    let name: String = Name(ZH_TW).fake();
    println!("name {:?}", name);

    // using convenient function without providing locale
    use fake::faker::lorem::en::*;
    let words: Vec<String> = Words(3..5).fake();
    println!("words {:?}", words);

    // using macro to generate nested collection
    let name_vec = fake::vec![String as Name(EN); 4, 3..5, 2];
    println!("random nested vec {:?}", name_vec);

    // fixed seed rng
    let seed = [
        1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let ref mut r = StdRng::from_seed(seed);
    for _ in 0..5 {
        let v: usize = Faker.fake_with_rng(r);
        println!("value from fixed seed {}", v);
    }
}
