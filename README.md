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

If you want Http faker features:
```toml
[dependencies.fake]
version = "*"
features = ["http"]
```

## Usage

```rust
// Use macro style
#[macro_use]
extern crate fake;

fake!(Internet.free_email);
fake!(Company.name);
fake!(Name.name);
fake!(Name.name in zh_tw);

// Custom fake string
fn to_lowercase<S: Into<String>>(s: S) -> String {
    s.into().to_lowercase()
}
fake!("{} - {}", [Name.name | to_lowercase], [expr fake!(Name.name).to_lowercase()]);
fake!("{} - {} - {}", [Name.name], [Name.name in zh_tw], [Number.number(10)]);
fake!(r#"{{"name": "{x}", "chinese_name": "{y}"}}"#, [y = Name.name in zh_tw], [x = Name.name]);
fake!(r#"http://{domain}.{domain_suffix}/user/{username}.png?size={size}x{size}"#,
      [domain = Name.last_name | to_lowercase],
      [domain_suffix = Internet.domain_suffix],
      [username = Name.first_name | to_lowercase],
      [size = expr [512, 256, 128][gen_range(0, 3)]]);

// Use function call style
use fake::faker::*;

Faker::free_email();

// In case multiple candidates available
<Faker as Company>::name();
<Faker as Name>::name();

// Switch locales
use fake::locales::zh_tw;
println!("{}", <zh_tw::Faker as Name>::name());
```

## Lorem

```rust
println!("{:?}", fake!(Lorem.word));
println!("{:?}", fake!(Lorem.words(10)));
println!("{:?}", fake!(Lorem.sentence(4, 6)));
println!("{:?}", fake!(Lorem.sentences(10)));
println!("{:?}", fake!(Lorem.paragraph(7, 3)));
println!("{:?}", fake!(Lorem.paragraphs(3)));
```

## Name

```rust
println!("{:?}", fake!(Name.first_name));
println!("{:?}", fake!(Name.last_name));
println!("{:?}", fake!(Name.name));
println!("{:?}", fake!(Name.name_with_middle));
println!("{:?}", fake!(Name.title_descriptor));
println!("{:?}", fake!(Name.title_level));
println!("{:?}", fake!(Name.title_job));
println!("{:?}", fake!(Name.title));

println!("{}", fake!(Name.first_name in zh_tw));
println!("{}", fake!(Name.last_name in zh_tw));
println!("{}", fake!(Name.name in zh_tw));
```

## Number

```rust
println!("{:?}", fake!(Number.digit));
println!("{:?}", fake!(Number.number(10)));
println!("{:?}", fake!(Number.between(5, 10)));
println!("{:?}", fake!(Number.between(5.0_f32, 10.0_f32)));
```

## Boolean

```rust
println!("{:?}", fake!(Boolean.boolean));
```

## Internet

```rust
println!("{:?}", fake!(Internet.free_email_provider));
println!("{:?}", fake!(Internet.domain_suffix));
println!("{:?}", fake!(Internet.user_name));
println!("{:?}", fake!(Internet.free_email));
println!("{:?}", fake!(Internet.safe_email));
println!("{:?}", fake!(Internet.ip));
println!("{:?}", fake!(Internet.ipv4));
println!("{:?}", fake!(Internet.ipv6));
println!("{:?}", fake!(Internet.color));
println!("{:?}", fake!(Internet.user_agent);
```

## HTTP
```rust
// return status code with RFC
println!("{:?}", fake!(Http.status_code));
println!("{:?}", fake!(Http.status_code).canonical_reason());

// return status code within (100, 600]
println!("{:?}", fake!(Http.all_status_code));
println!("{:?}", fake!(Http.all_status_code).canonical_reason());

// http::StatusCode implement Dummy which return status code with RFC
use http;
println!("{:?}", http::StatusCode::dummy());
println!("{:?}", dummy!(http::StatusCode));
println!("{:?}", dummy!(Vec<http::StatusCode>));
```

## Company

```rust
println!("{:?}", fake!(Company.suffix));
println!("{:?}", fake!(Company.name));
println!("{:?}", fake!(Company.buzzword));
println!("{:?}", fake!(Company.catch_phase));
println!("{:?}", fake!(Company.bs));
println!("{:?}", fake!(Company.profession));
println!("{:?}", fake!(Company.industry));
```

## Address

```rust
println!("{:?}", fake!(Address.time_zone));
println!("{:?}", fake!(Address.city_prefix));
println!("{:?}", fake!(Address.city_suffix));
println!("{:?}", fake!(Address.street_suffix));
println!("{:?}", fake!(Address.state));
println!("{:?}", fake!(Address.state_abbr));
println!("{:?}", fake!(Address.city));
println!("{:?}", fake!(Address.street_name));
println!("{:?}", fake!(Address.building_number));
println!("{:?}", fake!(Address.street_address));
println!("{:?}", fake!(Address.secondary_address));
println!("{:?}", fake!(Address.zip));
println!("{:?}", fake!(Address.postcode));
println!("{:?}", fake!(Address.latitude));
println!("{:?}", fake!(Address.longitude));
```

## Phone Number

```rust
println!("{:?}", fake!(PhoneNumber.phone_number));
// N => [1..9], # => [0..9]
println!("{:?}", fake!(PhoneNumber.phone_number_with_format("N###-####")));
println!("{:?}", fake!(PhoneNumber.cell_number));
```

## Date/Time

```rust
use chrono::prelude::*;

let early = Utc.ymd(2010, 4, 20).and_hms(11, 11, 11);
let late = Utc.ymd(2020, 6, 5).and_hms(9, 32, 33);
let now = Utc::now();
let fmt = "%a %b %e %T %Y %:z";
let now_str = now.format(fmt).to_string();

println!("{:?}", fake!(Chrono.time(Some("%I.%M.%S %p")));
println!("{:?}", fake!(Chrono.date(Some("%A %Y.%m.%d")));
println!("{:?}", fake!(Chrono.datetime(None)));
println!("{:?}", fake!(Chrono.before(Some(fmt), now_str)));
println!("{:?}", fake!(Chrono.after(Some(fmt), now_str)));
println!(
    "{:?}",
    fake!(Chrono.between(
        Some(fmt),
        &early.format(fmt).to_string(),
        &late.format(fmt).to_string()
    ))
);
println!(
    "{:?}",
    fake!(
        Chrono.between(
            None,
            &early.to_rfc3339(),
            &late.to_rfc3339()
        )
    )
);
```

## Dummy

```rust
// dummy macro take T: Dummy
println!("{:?}", dummy!(i32));
println!("{:?}", dummy!(Vec<Vec<i32>>));
```

## Contributing

### What can you help

1.  Add locales
2.  Add new faker
3.  Report bugs
4.  Fix Issues

### How

1.  Fork the repo.
2.  Add a test for your change.
3.  Make the test. `cargo test`
4.  Push to your fork and submit a pull request.

## LICENSE

The MIT License (MIT)
