use fake::decimal::Decimal::*;
use fake::faker::boolean::en::*;
use fake::faker::company::en::*;
use fake::faker::name::en::*;
use fake::Dummy;
use fake::{Fake, Faker};
use fake::uuid::UUIDv4;


#[derive(Debug, Dummy)]
pub enum OrderStatus {
    Completed,
    Cancelled,
}

#[derive(Debug, Dummy)]
pub struct Order {
    #[dummy(faker = "1000..")]
    pub order_id: usize,

    #[dummy(faker = "Name()")]
    pub customer: String,

    #[dummy(faker = "(Faker, 3..5)")]
    pub items: Vec<Item>,

    #[dummy(faker = "Boolean(70)")]
    pub paid: bool,

    pub status: OrderStatus,

    #[dummy(faker = "UUIDv4")]
    pub uuid: uuid::Uuid,

    #[dummy(faker = "PositiveDecimal")]
    pub total: rust_decimal::Decimal,
}

#[derive(Debug, Dummy)]
pub struct Item {
    #[dummy(faker = "1..100")]
    pub product_id: usize,

    pub qty: u8,

    #[dummy(faker = "CompanyName()")]
    pub company: String,

    #[dummy(faker = "PositiveDecimal")]
    pub price: rust_decimal::Decimal,
}

#[allow(dead_code)]
#[derive(Debug, Dummy)]
enum Message {
    Quit,
    Move {
        #[dummy(faker = "1..100")]
        x: i32,
        #[dummy(faker = "1..100")]
        y: i32,
    },
    Write(#[dummy(faker = "Buzzword()")] String),
    ChangeColor(
        #[dummy(faker = "1..100")] i32,
        #[dummy(faker = "1..100")] i32,
        #[dummy(faker = "1..100")] i32,
    ),
    Order(Order),
}

#[derive(Debug, Dummy)]
struct UnitStruct;

#[derive(Debug, Dummy)]
struct EmptyStruct {}

// #[derive(Debug, Dummy)]
// enum EmptyEnum {}

#[derive(Debug, Dummy)]
struct NewType(#[dummy(faker = "1..100")] usize, String);

fn main() {
    let order: Order = Faker.fake();
    println!("{:#?}", order);

    let msg: Message = Faker.fake();
    println!("{:#?}", msg);

    let v: UnitStruct = Faker.fake();
    println!("{:#?}", v);

    let v: EmptyStruct = Faker.fake();
    println!("{:#?}", v);

    // let v: EmptyEnum = Faker.fake();
    // println!("{:#?}", v);

    let v: NewType = Faker.fake();
    println!("{:#?}", v);

    let v: uuid::Uuid = Faker.fake();
    println!("{:#?}", v);
}
