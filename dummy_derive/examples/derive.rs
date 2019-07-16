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
