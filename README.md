# Fake

![Rust](https://github.com/cksac/fake-rs/workflows/Rust/badge.svg)
[![Docs Status](https://docs.rs/fake/badge.svg)](https://docs.rs/fake)
[![Latest Version](https://img.shields.io/crates/v/fake.svg)](https://crates.io/crates/fake)

A Rust library and command line tool for generating fake data in different languages. Currently supports:

| Language              | Code  |
|-----------------------|-------|
| English               | en    |
| French                | fr_fR |
| Arabic                | ar_sA |
| Traditional Chinese   | zh_tw |
| Simplified Chinese    | zh_cn |
| Japanese              | ja_jp |
| Portugese (Brazilian) | pt_br |


## Installation

### Library:

```toml
[dependencies]
fake = { version = "3.0.1", features = ["derive"] }
```

Available library features:

- `derive`: if you want to use `#[derive(Dummy)]`
- supported crates feature flags:
  - `chrono`
  - `chrono-tz`
  - `http`
  - `ulid`
  - `uuid`
  - `bigdecimal` (via `bigdecimal-rs`)
  - `rust_decimal`
  - `random_color`
  - `geo`
  - `semver`
  - `serde_json`
  - `time`
  - `zerocopy`
  - `glam`
  - `url`
  - `indexmap`
- `always-true-rng`: expose AlwaysTrueRng
- `maybe-non-empty-collections`: allow to use AlwaysTrueRng to generate non-empty collections

### CLI:
`cargo install --features=cli --git https://github.com/cksac/fake-rs.git`

Access cli using `fake` command. Below are the currently available fake generators.

```shell
❯ fake
An easy to use library and command line for generating fake data like name, number, address, lorem, dates, etc.

Usage: fake [OPTIONS] [COMMAND]

Commands:
  CityPrefix            
  CitySuffix            
  CityName              
  CountryName           
  CountryCode           
  StreetSuffix          
  StreetName            
  TimeZone              
  StateName             
  StateAbbr             
  SecondaryAddressType  
  SecondaryAddress      
  ZipCode               
  PostCode              
  BuildingNumber        
  Latitude              
  Longitude             
  Geohash               
  Isbn                  
  Isbn10                
  Isbn13                
  CreditCardNumber      
  CompanySuffix         
  CompanyName           
  Buzzword              
  BuzzwordMiddle        
  BuzzwordTail          
  CatchPhrase           
  BsVerb                
  BsAdj                 
  BsNoun                
  Bs                    
  Profession            
  Industry              
  FreeEmailProvider     
  DomainSuffix          
  FreeEmail             
  SafeEmail             
  Username              
  Password              
  IPv4                  
  IPv6                  
  IP                    
  MACAddress            
  UserAgent             
  Seniority             
  Field                 
  Position              
  Word                  
  Words                 
  Sentence              
  Sentences             
  Paragraph             
  Paragraphs            
  FirstName             
  LastName              
  Title                 
  Suffix                
  Name                  
  NameWithTitle         
  PhoneNumber           
  CellNumber            
  FilePath              
  FileName              
  FileExtension         
  DirPath               
  MimeType              
  Semver                
  SemverStable          
  SemverUnstable        
  CurrencyCode          
  CurrencyName          
  CurrencySymbol        
  Bic                   
  Isin                  
  HexColor              
  RgbColor              
  RgbaColor             
  HslColor              
  HslaColor             
  Color                 
  Time                  
  Date                  
  DateTime              
  RfcStatusCode         
  ValidStatusCode       
  help                  Print this message or the help of the given subcommand(s)

Options:
  -r, --repeat <repeat>  [default: 1]
  -l, --locale <locale>  [default: EN]
  -h, --help             Print help
  -V, --version          Print version

```

## Usage

### In rust code
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

#[derive(Debug, Dummy)]
struct Bar<T> {
    field: Vec<T>,
}

fn main() {
    // type derived Dummy
    let f: Foo = Faker.fake();
    println!("{:?}", f);

    let b: Bar<Foo> = Faker.fake();
    println!("{:?}", b);

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

## Command line

Generate random name (defaults to EN locale)
```shell
❯ ./fake Name
Generating 1 fakes for EN locale
Theresa Walker
```
Generate 5 chinese random names by mentioning locale to zh_cn
```shell
❯ ./fake -r5 -lzh_cn Name
Generating 5 fakes for ZH_CN locale
何丹华
尹雅瑾
于金福
郭雨珍
龙菲霞
```
Generate 5 random passwords with minimum 10 characters
```shell
❯ ./fake -r5 Password --min 10
Generating 5 fakes for EN locale
Q6eeXHfC3uzSRqtZwB
6fDHAOh3I7Ah77duLL
R8ygoTLmd4i1z1Z
5Uxj3RdEK5O4Af3ow
2XWsGT0lUaDnMZTb7
```
Arguments can be sent to fake generators like password that accept different ranges
```shell
❯ ./fake Password --help
Usage: fake Password [OPTIONS]

Options:
      --max <max>  [default: 20]
      --min <min>  [default: 10]
  -h, --help       Print help
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
NumberWithFormat<'a>(fmt: &'a str);
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
UserAgent();
```

## HTTP

```rust
RfcStatusCode();
ValidStatusCode();
```

## Color

```rust
HexColor();
RgbColor();
RgbaColor();
HslColor();
HslaColor();
Color();
```

## Company

```rust
CompanySuffix();
CompanyName();
Buzzword();
BuzzwordMiddle();
BuzzwordTail();
CatchPhrase();
BsVerb();
BsAdj();
BsNoun();
Bs();
Profession();
Industry();
```

## Currency

```rust
CurrencyCode();
CurrencyName();
CurrencySymbol();
```

## Creditcard

```rust
CreditCardNumber();
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

## Administrative

```rust
HealthInsuranceCode();
```

## Automotive

```rust
LicencePlate();
```

## Barcode

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
Isin();
```

### UUID

```rust
UUIDv1();
UUIDv3();
UUIDv4();
UUIDv5();
```

### Decimal

```rust
Decimal();
PositiveDecimal();
NegativeDecimal();
NoDecimalPoints();
```

### Bigdecimal

```rust
BigDecimal();
PositiveBigDecimal();
NegativeBigDecimal();
NoBigDecimalPoints();
```

## Utils
### Either
```rust
use fake::faker::phone_number::en::{CellNumber, PhoneNumber};
use fake::{utils::{either, WrappedVal}, Dummy, Fake, Faker};

#[derive(Debug, Dummy)]
struct Foo {
    #[dummy(faker = "either(PhoneNumber(), CellNumber())", wrapper = "WrappedVal")]
    phone_number: String,
}
```

# LICENSE

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.
