use fake::locales::{EN, ZH_TW};
use fake::{Fake, Faker};

fn lorem_faker() {
    use fake::faker::lorem::*;

    let val: String = Word(EN).fake();
    println!("{:?}", val);

    let val: Vec<String> = Words(EN, 3..5).fake();
    println!("{:?}", val);

    let val: String = Sentence(EN, 3..5).fake();
    println!("{:?}", val);

    let val: Vec<String> = Sentences(EN, 3..5).fake();
    println!("{:?}", val);

    let val: String = Paragraph(EN, 3..5).fake();
    println!("{:?}", val);

    let val: Vec<String> = Paragraphs(EN, 3..5).fake();
    println!("{:?}", val);
}

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

fn company_faker() {
    use fake::faker::company::*;

    let val: String = CompanySuffix(EN).fake();
    println!("{:?}", val);

    let val: String = CompanyName(EN).fake();
    println!("{:?}", val);

    let val: String = Buzzword(EN).fake();
    println!("{:?}", val);

    let val: String = BuzzwordMiddle(EN).fake();
    println!("{:?}", val);

    let val: String = BuzzwordTail(EN).fake();
    println!("{:?}", val);

    let val: String = CatchPhase(EN).fake();
    println!("{:?}", val);

    let val: String = BsVerb(EN).fake();
    println!("{:?}", val);

    let val: String = BsAdj(EN).fake();
    println!("{:?}", val);

    let val: String = BsNoun(EN).fake();
    println!("{:?}", val);

    let val: String = Bs(EN).fake();
    println!("{:?}", val);

    let val: String = Profession(EN).fake();
    println!("{:?}", val);

    let val: String = Industry(EN).fake();
    println!("{:?}", val);
}

fn internet_faker() {
    use fake::faker::internet::*;

    let val: String = FreeEmailProvider(EN).fake();
    println!("{:?}", val);

    let val: String = DomainSuffix(EN).fake();
    println!("{:?}", val);

    let val: String = FreeEmail(EN).fake();
    println!("{:?}", val);

    let val: String = SafeEmail(EN).fake();
    println!("{:?}", val);

    let val: String = Username(EN).fake();
    println!("{:?}", val);

    let val: String = Password(EN, 8..20).fake();
    println!("{:?}", val);

    let val: String = IPv4.fake();
    println!("{:?}", val);

    let val: String = IPv6.fake();
    println!("{:?}", val);

    let val: String = Color.fake();
    println!("{:?}", val);

    let val: String = UserAgent(EN).fake();
    println!("{:?}", val);
}

fn number_faker() {
    use fake::faker::number::*;

    let val: String = Digit(EN).fake();
    println!("{:?}", val);

    // ^: 1-9, #: 0-9
    let val: String = NumberWithFormat(EN, "^###").fake();
    println!("{:?}", val);

    let val: String = NumberWithFormat(EN, "FLAT 0# ^#/F").fake();
    println!("{:?}", val);
}

fn phone_number_faker() {
    use fake::faker::number::NumberWithFormat;
    use fake::faker::phone_number::*;

    let val: String = PhoneNumber(EN).fake();
    println!("{:?}", val);

    let val: String = CellNumber(EN).fake();
    println!("{:?}", val);

    // custom phone number format
    let val: String = NumberWithFormat(EN, "(+852) 6### ####").fake();
    println!("{:?}", val);
}

#[cfg(feature = "http")]
fn http_faker() {
    use fake::faker::http::*;
    use http::status::{InvalidStatusCode, StatusCode};

    let val: String = RfcStatusCode(EN).fake();
    println!("{:?}", val);

    let val: String = ValidStatusCode(EN).fake();
    println!("{:?}", val);

    let val: StatusCode = RfcStatusCode(EN).fake();
    println!("{:?}", val);

    let val: StatusCode = ValidStatusCode(EN).fake();
    println!("{:?}", val);

    let codes = [200, 401, 404, 500].as_ref();
    let val: StatusCode = codes.fake();
    println!("{:?}", val);

    let codes = [200, 401, 404, 500, 611].as_ref();
    let val: Result<StatusCode, InvalidStatusCode> = codes.fake();
    println!("{:?}", val);

    let val: StatusCode = Faker.fake();
    println!("{:?}", val);

    let val: http::Version = Faker.fake();
    println!("{:?}", val);
}

fn main() {
    lorem_faker();
    name_faker();
    job_faker();
    address_faker();
    bool_faker();
    company_faker();
    internet_faker();
    number_faker();
    phone_number_faker();

    #[cfg(feature = "http")]
    http_faker();
}
