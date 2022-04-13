# Fake

![Rust](https://github.com/cksac/fake-rs/workflows/Rust/badge.svg)
[![Docs Status](https://docs.rs/fake/badge.svg)](https://docs.rs/fake)
[![Latest Version](https://img.shields.io/crates/v/fake.svg)](https://crates.io/crates/fake)

A Rust library for generating fake data.

## Installation

Default (`rand` is required):
```toml
[dependencies]
fake = "2.4"
rand = "0.8"
```
If you want to use `#[derive(Dummy)]`:
```toml
fake = { version = "2.4", features=['derive']}
```
If you want the date and time features from `chrono`:
```toml
fake = { version = "2.4", features=['chrono']}
```
If you want `http` faker features:
```toml
fake = { version = "2.4", features=['http']}
```
If you want `uuid` faker features:
```toml
fake = { version = "2.4", features=['uuid']}
```
If you want `rust_decimal` faker features:
```toml
fake = { version = "2.4", features=['rust_decimal']}
```

## Usage

```rust
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
```

# Fakers with locale
## Lorem

```rust
Word();
Words(count: Range<usize>);
Sentence(count: Range<usize>);
Sentences(count: Range<usize>);
Paragraph(count: Range<usize>);
Paragraphs(count: Range<usize>);
```

## Name

```rust
FirstName();
LastName();
Title();
Suffix();
Name();
NameWithTitle();
```

## Number

```rust
Digit();
NumberWithFormat(fmt: &'static str);
```

## Boolean

```rust
Boolean(ratio: u8);
```

## Internet

```rust
FreeEmailProvider();
DomainSuffix();
FreeEmail();
SafeEmail();
Username();
Password(len_range: Range<usize>);
IPv4();
IPv6();
IP();
MACAddress();
Color();
UserAgent();
```

## HTTP
```rust
RfcStatusCode();
ValidStatusCode();
```

## Company

```rust
CompanySuffix();
CompanyName();
Buzzword();
BuzzwordMiddle();
BuzzwordTail();
CatchPhase();
BsVerb();
BsAdj();
BsNoun();
Bs();
Profession();
Industry();
```

## Address

```rust
CityPrefix();
CitySuffix();
CityName();
CountryName();
CountryCode();
StreetSuffix();
StreetName();
TimeZone();
StateName();
StateAbbr();
SecondaryAddressType();
SecondaryAddress();
ZipCode();
PostCode();
BuildingNumber();
Latitude();
Longitude();
Geohash(precision: u8);
```

### Automotive
```rust
LicencePlate();
```

### BareCode
```rust
Isbn();
Isbn13();
Isbn10();
```

## Phone Number

```rust
PhoneNumber();
CellNumber();
```

## Date/Time

```rust
Time();
Date();
DateTime();
Duration();
DateTimeBefore(dt: DateTime<Utc>);
DateTimeAfter(dt: DateTime<Utc>);
DateTimeBetween(start: DateTime<Utc>, end: DateTime<Utc>);
```

## Filesystem
```rust
FilePath();
FileName();
FileExtension();
DirPath();
```

### Finance
```rust
Bic();
```

### UUID
```rust
UUIDv1();
UUIDv3();
UUIDv4();
UUIDv5();
```

# LICENSE

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
