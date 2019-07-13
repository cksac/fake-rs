use fake::locales::{EN, ZH_TW};
use fake::Fake;

fn name_faker() {
    use fake::faker::name::*;

    let val: String = FirstName(EN).fake();
    println!("{:?}", val);

    let val: String = LastName(EN).fake();
    println!("{:?}", val);

    let val: String = Title(EN).fake();
    println!("{:?}", val);

    let val: String = Suffix(EN).fake();
    println!("{:?}", val);

    let val: String = Name(EN).fake();
    println!("{:?}", val);

    let val: String = NameWithTitle(EN).fake();
    println!("{:?}", val);

    let val: String = Name(ZH_TW).fake();
    println!("{:?}", val);

    let val: String = NameWithTitle(ZH_TW).fake();
    println!("{:?}", val);
}

fn job_faker() {
    use fake::faker::job::*;
    let val: String = Seniority(EN).fake();
    println!("{:?}", val);

    let val: String = Field(EN).fake();
    println!("{:?}", val);

    let val: String = Position(EN).fake();
    println!("{:?}", val);

    let val: String = Title(EN).fake();
    println!("{:?}", val);

    let val: Vec<String> = (Title(EN), 2..4).fake();
    println!("{:?}", val);
}

fn main() {
    name_faker();
    job_faker();
}
