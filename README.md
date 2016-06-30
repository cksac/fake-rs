# Fake
A library for generating fake data in Rust.

##Installation
Add fake to your Cargo.toml
```toml
[dependencies]
fake = "*"
```
## Usage
```rust
use fake::{Fake, Faker};
Faker::lorem_word();
```
## Lorem
```rust
use fake::*;

//use facade function in Fake Trait
//Convention: Faker::{trait}_{fn}(args)
println!("{:?}", Faker::lorem_word());
println!("{:?}", Faker::lorem_words(10));
println!("{:?}", Faker::lorem_sentence(4, 6));
println!("{:?}", Faker::lorem_sentences(10));
println!("{:?}", Faker::lorem_paragraph(7, 3));
println!("{:?}", Faker::lorem_paragraphs(3));

//use function of Lorem Trait
println!("{:?}", <Faker as Lorem>::word());
println!("{:?}", <Faker as Lorem>::words(10));
println!("{:?}", <Faker as Lorem>::sentence(4, 6));
println!("{:?}", <Faker as Lorem>::sentences(10));
println!("{:?}", <Faker as Lorem>::paragraph(7, 3));
println!("{:?}", <Faker as Lorem>::paragraphs(3));
```
## Name
```rust
use fake::*;

println!("{:?}", <Faker as Name>::first_name());
println!("{:?}", <Faker as Name>::last_name());
println!("{:?}", <Faker as Name>::name());
println!("{:?}", <Faker as Name>::name_with_middle());
println!("{:?}", <Faker as Name>::title_descriptor());
println!("{:?}", <Faker as Name>::title_level());
println!("{:?}", <Faker as Name>::title_job());
println!("{:?}", <Faker as Name>::title());

use fake::locales::zh_tw;
println!("{}", <zh_tw::Faker as Name>::first_name());
println!("{}", <zh_tw::Faker as Name>::last_name());
println!("{}", <zh_tw::Faker as Name>::name());
```
## Number
```rust
use fake::*;

println!("{:?}", <Faker as Number>::digit());
println!("{:?}", <Faker as Number>::number(10));
println!("{:?}", <Faker as Number>::between(5, 10));
println!("{:?}", <Faker as Number>::between(5.0_f32, 10.0_f32));
```
## Boolean
```rust
use fake::*;

println!("{:?}", <Faker as Boolean>::boolean());
```

##Contributing
1. Fork the repo.
3. Add a test for your change.
4. Make the test pass. `cargo test`
5. Push to your fork and submit a pull request.
