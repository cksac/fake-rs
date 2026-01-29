use fake::faker::phone_number::en::{CellNumber, PhoneNumber};
use fake::{
    utils::{either, WrappedVal},
    Dummy, Fake, Faker,
};

#[derive(Debug, Dummy)]
struct Foo {
    #[allow(unused)]
    #[dummy(faker = "either(PhoneNumber(), CellNumber())", wrapper = "WrappedVal")]
    phone_number: String,
}

fn main() {
    #[allow(clippy::disallowed_names)]
    for _ in 0..10 {
        let foo: Foo = Faker.fake();
        println!("{foo:?}");
    }
}
