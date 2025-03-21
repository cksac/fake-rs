use fake::faker::lorem::raw::Word;
use fake::locales::{DynData, Locale, EN};
use fake::{Fake, ResultFaker, StringFaker};

fn main() {
    let val: String = Word(Locale::en).fake();
    println!("{:?}", val);

    let val: String = Word(Box::new(Locale::en) as Box<dyn DynData>).fake();
    println!("{:?}", val);

    let val: String = Word(Box::new(EN) as Box<dyn DynData>).fake();
    println!("{:?}", val);
}
