# Fake
[![Build Status](https://travis-ci.org/cksac/fake-rs.svg?branch=master)](https://travis-ci.org/cksac/fake-rs)
A library for generating fake data in Rust.

##Installation
Add fake to your Cargo.toml
```toml
[dependencies]
fake = "*"
```
## Usage
```rust
use fake::faker::*;
Faker::name_with_middle();
```
## Lorem
```rust
println!("{:?}", Faker::word());
println!("{:?}", Faker::words(10));
println!("{:?}", Faker::sentence(4, 6));
println!("{:?}", Faker::sentences(10));
println!("{:?}", Faker::paragraph(7, 3));
println!("{:?}", Faker::paragraphs(3));
```
## Name
```rust
println!("{:?}", Faker::first_name());
println!("{:?}", Faker::last_name());
println!("{:?}", Faker::name());
println!("{:?}", Faker::name_with_middle());
println!("{:?}", Faker::title_descriptor());
println!("{:?}", Faker::title_level());
println!("{:?}", Faker::title_job());
println!("{:?}", Faker::title());

use fake::locales::zh_tw;
println!("{}", zh_tw::Faker::first_name());
println!("{}", zh_tw::Faker::last_name());
println!("{}", zh_tw::Faker::name());
```
## Number
```rust
println!("{:?}", Faker::digit());
println!("{:?}", Faker::number(10));
println!("{:?}", Faker::between(5, 10));
println!("{:?}", Faker::between(5.0_f32, 10.0_f32));
```
## Boolean
```rust
println!("{:?}", Faker::boolean());
```
## Internet
```rust
println!("{:?}", Faker::free_email_provider());
println!("{:?}", Faker::domain_suffix());
println!("{:?}", Faker::user_name());
println!("{:?}", Faker::free_email());
println!("{:?}", Faker::safe_email());
```

##Contributing
1. Fork the repo.
3. Add a test for your change.
4. Make the test and clippy lint pass. `cargo test --features "dev"`
5. Push to your fork and submit a pull request.
