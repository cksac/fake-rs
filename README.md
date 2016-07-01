# Fake
[![Build Status](https://travis-ci.org/cksac/fake-rs.svg?branch=master)](https://travis-ci.org/cksac/fake-rs)
[![Latest Version](https://img.shields.io/crates/v/fake.svg)](https://crates.io/crates/fake)

A Rust library for generating fake data. Currently only works in Rust nightly.

##Installation
Add fake to your Cargo.toml
```toml
[dependencies]
fake = "*"
```
## Usage
```rust
use fake::faker::*;

Faker::free_email();

// In case multiple candidates available
<Faker as Company>::name();
<Faker as Name>::name();
```
## Lorem
```rust
println!("{:?}", <Faker as Lorem>::word());
println!("{:?}", <Faker as Lorem>::words(10));
println!("{:?}", <Faker as Lorem>::sentence(4, 6));
println!("{:?}", <Faker as Lorem>::sentences(10));
println!("{:?}", <Faker as Lorem>::paragraph(7, 3));
println!("{:?}", <Faker as Lorem>::paragraphs(3));
```
## Name
```rust
println!("{:?}", <Faker as Name>::first_name());
println!("{:?}", <Faker as Name>::last_name());
println!("{:?}", <Faker as Name>::name());
println!("{:?}", <Faker as Name>::name_with_middle());
println!("{:?}", <Faker as Name>::title_descriptor());
println!("{:?}", <Faker as Name>::title_level());
println!("{:?}", <Faker as Name>::title_job());
println!("{:?}", <Faker as Name>::title());

use super::locales::zh_tw;
println!("{}", <zh_tw::Faker as Name>::first_name());
println!("{}", <zh_tw::Faker as Name>::last_name());
println!("{}", <zh_tw::Faker as Name>::name());
```
## Number
```rust
println!("{:?}", <Faker as Number>::digit());
println!("{:?}", <Faker as Number>::number(10));
println!("{:?}", <Faker as Number>::between(5, 10));
println!("{:?}", <Faker as Number>::between(5.0_f32, 10.0_f32));
```
## Boolean
```rust
println!("{:?}", <Faker as Boolean>::boolean());
```
## Internet
```rust
println!("{:?}", <Faker as Internet>::free_email_provider());
println!("{:?}", <Faker as Internet>::domain_suffix());
println!("{:?}", <Faker as Internet>::user_name());
println!("{:?}", <Faker as Internet>::free_email());
println!("{:?}", <Faker as Internet>::safe_email());
```
## Company
```rust
println!("{:?}", <Faker as Company>::suffix());
println!("{:?}", <Faker as Company>::name());
println!("{:?}", <Faker as Company>::buzzword());
println!("{:?}", <Faker as Company>::catch_phase());
println!("{:?}", <Faker as Company>::bs());
println!("{:?}", <Faker as Company>::profession());
println!("{:?}", <Faker as Company>::industry());
```

## Contributing
1. Fork the repo.
3. Add a test for your change.
4. Make the test and clippy lint pass. `cargo test --features "dev"`
5. Push to your fork and submit a pull request.
