# Fake

[![Build Status](https://travis-ci.org/cksac/fake-rs.svg?branch=master)](https://travis-ci.org/cksac/fake-rs)
[![Latest Version](https://img.shields.io/crates/v/fake.svg)](https://crates.io/crates/fake)

A Rust library for generating fake data.

## Installation

Default:
```toml
[dependencies]
fake = "*"
```
If you want the date and time features from chrono:
```toml
[dependencies.fake]
version = "*"
features = ["chrono"]
```

If you want Http faker features:
```toml
[dependencies.fake]
version = "*"
features = ["http"]
```

## Usage

```rust
#[macro_use]
extern crate fake;
use fake::{Fake, Faker};

fn main() {
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
    println!("name {:?}", words);

    // using macro to generate nested collection
    let name_vec = fake::vec![String as Name(EN); 4, 3..5, 2];
    println!("random nested vec {:?}", name_vec);    
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

# LICENSE

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.