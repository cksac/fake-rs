use fake::faker::boolean::en::*;
use fake::faker::company::en::*;
use fake::faker::name::en::*;
// use fake::faker::internet::en::*;
// use fake::UuidConfig;
use fake::Dummy;
use fake::{Fake, Faker};


#[derive(Debug, Dummy)]
enum OrderStatus {
    Completed,
    Cancelled,
}

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
    
    status: OrderStatus,

    // #[dummy(faker = "Uuid(UuidConfig::V4())")]
    // uuid: uuid::Uuid,
}

#[derive(Debug, Dummy)]
pub struct Item {
    #[dummy(faker = "1..100")]
    product_id: usize,

    qty: u8,

    #[dummy(faker = "CompanyName()")]
    company: String,
}

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
