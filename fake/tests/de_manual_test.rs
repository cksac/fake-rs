use fake::faker::address::de_de::SecondaryAddress;
use fake::faker::address::raw::{CityName, StateName, StreetName, ZipCode};
use fake::faker::company::raw::CompanyName;
use fake::locales::DE_DE;
use fake::Fake;

#[allow(unused)]
#[derive(Debug)]
pub struct Company {
    name: String,
    address_line_1: String,
    address_line_2: String,
    zipcode: String,
    city: String,
    state: String,
}

#[test]
fn de_manual_test() {
    for _ in 1..10 {
        let c = Company {
            name: CompanyName(DE_DE).fake(),
            address_line_1: format!(
                "{} {}",
                StreetName(DE_DE).fake::<String>(),
                (1..100).fake::<u16>()
            ),
            address_line_2: SecondaryAddress().fake(),
            zipcode: ZipCode(DE_DE).fake(),
            city: CityName(DE_DE).fake(),
            state: StateName(DE_DE).fake(),
        };
        println!("{:#?}", c);
    }
}
#[test]
fn de_manual_city_test() {
    for _ in 1..10 {
        let city: String = CityName(DE_DE).fake();
        println!("{:#?}", city);
    }
}
