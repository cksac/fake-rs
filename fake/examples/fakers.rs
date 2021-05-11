use fake::locales::{EN, ZH_CN, ZH_TW};
use fake::Fake;

fn lorem_faker() {
    use fake::faker::lorem::raw::*;

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
    use fake::faker::name::raw::*;

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

    let val: String = Name(ZH_CN).fake();
    println!("{:?}", val);

    let val: String = NameWithTitle(ZH_CN).fake();
    println!("{:?}", val);
}

fn job_faker() {
    use fake::faker::job::raw::*;
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
    use fake::faker::address::raw::*;

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

    let val: f32 = Latitude(EN).fake();
    println!("{:?}", val);

    let val: f64 = Latitude(EN).fake();
    println!("{:?}", val);

    let val: f32 = Longitude(EN).fake();
    println!("{:?}", val);

    let val: f64 = Longitude(EN).fake();
    println!("{:?}", val);
}

fn bool_faker() {
    use fake::faker::boolean::raw::*;

    use fake::faker::boolean::en;
    let b = en::Boolean(50);
    for _ in 0..5 {
        let val: bool = b.fake();
        println!("{:?}", val);
    }

    //50% true
    let b = Boolean(EN, 50);
    for _ in 0..5 {
        let val: bool = b.fake();
        println!("{:?}", val);
    }

    // 0% true
    let b = Boolean(EN, 0);
    for _ in 0..5 {
        let val: bool = b.fake();
        println!("{:?}", val);
    }

    // 100% true
    let b = Boolean(EN, 100);
    for _ in 0..5 {
        let val: bool = b.fake();
        println!("{:?}", val);
    }
}

fn company_faker() {
    use fake::faker::company::raw::*;

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
    use fake::faker::internet::raw::*;

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

    let val: String = IPv4(EN).fake();
    println!("{:?}", val);

    let val: String = IPv6(EN).fake();
    println!("{:?}", val);

    let val: String = MACAddress(EN).fake();
    println!("{:?}", val);

    let val: String = Color(EN).fake();
    println!("{:?}", val);

    let val: String = UserAgent(EN).fake();
    println!("{:?}", val);
}

fn number_faker() {
    use fake::faker::number::raw::*;

    let val: String = Digit(EN).fake();
    println!("{:?}", val);

    // ^: 1-9, #: 0-9
    let val: String = NumberWithFormat(EN, "^###").fake();
    println!("{:?}", val);

    let val: String = NumberWithFormat(EN, "FLAT 0# ^#/F").fake();
    println!("{:?}", val);
}

fn phone_number_faker() {
    use fake::faker::number::raw::NumberWithFormat;
    use fake::faker::phone_number::raw::*;

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
    use fake::faker::http::raw::*;
    use fake::Faker;
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

#[cfg(feature = "chrono")]
fn chrono_faker() {
    use chrono::Utc;
    use fake::faker::chrono::raw::*;
    use fake::Faker;

    let val: chrono::NaiveTime = Time(EN).fake();
    println!("{:?}", val);

    let val: String = Time(EN).fake();
    println!("{:?}", val);

    let val: chrono::NaiveDate = Date(EN).fake();
    println!("{:?}", val);

    let val: String = Date(EN).fake();
    println!("{:?}", val);

    let val: chrono::NaiveDateTime = DateTime(EN).fake();
    println!("{:?}", val);

    let val: chrono::DateTime<Utc> = DateTime(EN).fake();
    println!("{:?}", val);

    let val: String = DateTime(EN).fake();
    println!("{:?}", val);

    let val: chrono::Duration = Duration(EN).fake();
    println!("{}", val);

    let val: chrono::Duration = Faker.fake();
    println!("{}", val);

    let start_dt: chrono::DateTime<Utc> = DateTimeBefore(EN, Utc::now()).fake();
    println!("{}", start_dt);

    let end_dt: chrono::DateTime<Utc> = DateTimeAfter(EN, Utc::now()).fake();
    println!("{}", end_dt);

    let between: chrono::DateTime<Utc> = DateTimeBetween(EN, start_dt, end_dt).fake();
    println!("{}", between);
}

fn filesystem_faker() {
    use fake::faker::filesystem::raw::*;
    use std::path::PathBuf;

    let val: String = FilePath(EN).fake();
    println!("{:?}", val);

    let val: PathBuf = FilePath(EN).fake();
    println!("{:?}", val);

    let val: String = FileName(EN).fake();
    println!("{:?}", val);

    let val: String = FileExtension(EN).fake();
    println!("{:?}", val);

    let val: String = DirPath(EN).fake();
    println!("{:?}", val);

    let val: PathBuf = DirPath(EN).fake();
    println!("{:?}", val);
}

fn currency_faker() {
    use fake::faker::currency::raw::*;

    let val: String = CurrencyCode(EN).fake();
    println!("{:?}", val);

    let val: String = CurrencyName(EN).fake();
    println!("{:?}", val);

    let val: String = CurrencySymbol(EN).fake();
    println!("{:?}", val);
}

fn barecode_faker() {
    use fake::faker::barecode::raw::*;

    let val: String = Isbn13(EN).fake();
    println!("{}", val);

    let val: String = Isbn10(EN).fake();
    println!("{}", val);

    let val: String = Isbn(EN).fake();
    println!("{}", val);
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
    filesystem_faker();
    currency_faker();
    barecode_faker();

    #[cfg(feature = "http")]
    http_faker();

    #[cfg(feature = "chrono")]
    chrono_faker();
}
