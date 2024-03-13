use fake::faker::phone_number::en::{CellNumber, PhoneNumber};
use fake::{
    utils::{either, WrappedVal},
    Dummy, Fake, Faker,
};

#[derive(Debug, Dummy)]
struct Foo {
    #[dummy(faker = "either(PhoneNumber(), CellNumber())", wrapper = "WrappedVal")]
    phone_number: String,
}

fn main() {
    for _ in 0..10 {
        let foo: Foo = Faker.fake();
        println!("{:?}", foo);
    }
}
