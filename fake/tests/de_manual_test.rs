use fake::faker::address::raw::{CityName, StateName, StreetName, ZipCode};
use fake::faker::company::raw::CompanyName;
use fake::locales::DE_DE;
use fake::Fake;
#[allow(unused)]
#[derive(Debug)]
pub struct Company {
    name: String,
    street: String,
    zipcode: String,
    city: String,
    state: String,
}

#[test]
fn de_manual_test() {
    let c = Company {
        name: CompanyName(DE_DE).fake(),
        street: StreetName(DE_DE).fake(),
        zipcode: ZipCode(DE_DE).fake(),
        city: CityName(DE_DE).fake(),
        state: StateName(DE_DE).fake(),
    };
    println!("{:?}", c);
}
