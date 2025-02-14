# Dummy

[![Latest Version](https://img.shields.io/crates/v/dummy.svg)](https://crates.io/crates/dummy)

This crate provide derive(Dummy) macros. use it via `fake` crate with derive feature

## Installation

```toml
[dependencies]
fake = { version = "3", features=["derive"] }
```

## Usage

```rust
use fake::faker::boolean::en::*;
use fake::faker::company::en::*;
use fake::faker::name::en::*;
use fake::Dummy;
use fake::{Fake, Faker};

#[derive(Debug, Dummy)]
pub struct Order {
    #[dummy(faker = "1000..")]
    order_id: usize,

    #[dummy(faker = "Name()")]
    customer: String,

    #[dummy(faker = "(Faker, 3..5)")]
    items: Vec<Item>,

    #[dummy(faker = "Boolean(70)")]
    paid: bool,
}

#[derive(Debug, Dummy)]
pub struct Item {
    #[dummy(faker = "1..100")]
    product_id: usize,

    qty: u8,

    #[dummy(faker = "CompanyName()")]
    company: String,
}

fn main() {
    let order: Order = Faker.fake();
    println!("{:#?}", order);
}
```

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.
