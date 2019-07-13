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

fn address_faker() {
    use fake::faker::address::*;

    let val: String = CityPrefix(EN).fake();
    println!("{:?}", val);

    let val: String = CitySuffix(EN).fake();
    println!("{:?}", val);

    let val: String = CityName(EN).fake();
    println!("{:?}", val);

    let val: String = CountryName(EN).fake();
    println!("{:?}", val);

    let val: String = CountryCode(EN).fake();
    println!("{:?}", val);

    let val: String = StateName(EN).fake();
    println!("{:?}", val);

    let val: String = StateAbbr(EN).fake();
    println!("{:?}", val);

    let val: String = StreetSuffix(EN).fake();
    println!("{:?}", val);

    let val: String = StreetName(EN).fake();
    println!("{:?}", val);

    let val: String = SecondaryAddressType(EN).fake();
    println!("{:?}", val);

    let val: String = SecondaryAddress(EN).fake();
    println!("{:?}", val);

    let val: String = ZipCode(EN).fake();
    println!("{:?}", val);

    let val: String = PostCode(EN).fake();
    println!("{:?}", val);

    let val: String = BuildingNumber(EN).fake();
    println!("{:?}", val);

    let val: String = TimeZone(EN).fake();
    println!("{:?}", val);

    let val: f32 = Latitude.fake();
    println!("{:?}", val);

    let val: f64 = Latitude.fake();
    println!("{:?}", val);

    let val: f32 = Longitude.fake();
    println!("{:?}", val);

    let val: f64 = Longitude.fake();
    println!("{:?}", val);
}

fn bool_faker() {
    use fake::faker::boolean::*;

    //50% true
    let b = Boolean(50);
    for _ in 0..5 {
        let val: bool = b.fake();
        println!("{:?}", val);
    }

    // 0% true
    let b = Boolean(0);
    for _ in 0..5 {
        let val: bool = b.fake();
        println!("{:?}", val);
    }    

    // 100% true
    let b = Boolean(100);
    for _ in 0..5 {
        let val: bool = b.fake();
        println!("{:?}", val);
    }      
}

fn main() {
    name_faker();
    job_faker();
    address_faker();
    bool_faker();
}
