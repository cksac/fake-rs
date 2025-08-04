#![allow(dead_code)]

use fake::decimal::*;
use fake::faker::boolean::en::*;
use fake::faker::company::en::*;
use fake::faker::lorem::en::*;
use fake::faker::name::en::*;
use fake::ferroid::*;
use fake::utils::{either, WrappedVal};
use fake::uuid::UUIDv4;
use fake::Dummy;
use fake::{Fake, Faker};

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
    #[dummy(skip)]
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

#[allow(dead_code)]
#[derive(Debug, Dummy)]
struct ExprStruct {
    #[dummy(faker = "1..100")]
    pub product_id: usize,
    #[dummy(expr = "\"Base\".into()")]
    pub fixed_value: String,
}

#[allow(dead_code)]
#[derive(Debug, Dummy)]
struct DefaultStruct {
    #[dummy(faker = "1..100")]
    pub product_id: usize,
    #[dummy(default)]
    pub fixed_value: String,
}

#[derive(Debug, Dummy)]
struct UnitStruct;

#[derive(Debug, Dummy)]
struct EmptyStruct {}

#[derive(Debug, Dummy)]
enum EmptyEnum {}

#[derive(Debug, Dummy)]
struct NewTypeTuple(#[dummy(faker = "1..100")] usize, String);

#[derive(Debug, Dummy)]
struct NewTypeWord(#[dummy(faker = "Word()")] String);

#[derive(Debug, Dummy)]
struct NewTypeWords(#[dummy(faker = "Words(4..10)")] Vec<String>);

#[derive(Debug, Dummy)]
struct NewTypeSentence(#[dummy(faker = "Sentence(4..10)")] String);

#[derive(Debug, Dummy)]
#[allow(dead_code)]
struct MyStruct<T> {
    field: Vec<T>,
}

#[allow(dead_code)]
#[derive(Debug, Dummy)]
struct FakerWrapperStruct {
    #[dummy(faker = "either(Buzzword(), CompanyName())", wrapper = "WrappedVal")]
    pub val: String,
}

#[derive(Debug, Dummy)]
struct LogEntry {
    #[dummy(faker = "FerroidULID")]
    pub ulid: ferroid::ULID,

    #[dummy(faker = "FerroidTwitterId")]
    pub twitter_id: ferroid::SnowflakeTwitterId,

    #[dummy(faker = "FerroidInstagramId")]
    pub instagram_id: ferroid::SnowflakeInstagramId,

    #[dummy(faker = "FerroidMastodonId")]
    pub mastodon_id: ferroid::SnowflakeMastodonId,

    #[dummy(faker = "FerroidDiscordId")]
    pub discord_id: ferroid::SnowflakeDiscordId,
}

#[derive(Debug, Dummy)]
struct LogEntryBase32 {
    #[dummy(faker = "FerroidULID")]
    pub ulid: String,

    #[dummy(faker = "FerroidTwitterId")]
    pub twitter_id: String,

    #[dummy(faker = "FerroidInstagramId")]
    pub instagram_id: String,

    #[dummy(faker = "FerroidMastodonId")]
    pub mastodon_id: String,

    #[dummy(faker = "FerroidDiscordId")]
    pub discord_id: String,
}

fn main() {
    let order: Order = Faker.fake();
    println!("{:#?}", order);

    let msg: Message = Faker.fake();
    println!("{:#?}", msg);

    let v: UnitStruct = Faker.fake();
    println!("{:#?}", v);

    let v: EmptyStruct = Faker.fake();
    println!("{:#?}", v);

    // This causes "any code following this expression is unreachable"
    // let v: EmptyEnum = Faker.fake();
    // println!("{:#?}", v);

    let v: NewTypeTuple = Faker.fake();
    println!("{:#?}", v);

    let v: NewTypeWord = Faker.fake();
    println!("{:#?}", v);

    let v: NewTypeWords = Faker.fake();
    println!("{:#?}", v);

    let v: NewTypeSentence = Faker.fake();
    println!("{:#?}", v);

    let v: uuid::Uuid = Faker.fake();
    println!("{:#?}", v);

    let v: ExprStruct = Faker.fake();
    println!("{:#?}", v);

    let v: DefaultStruct = Faker.fake();
    println!("{:#?}", v);

    let v: MyStruct<u32> = Faker.fake();
    println!("{:#?}", v);

    let v: FakerWrapperStruct = Faker.fake();
    println!("{:#?}", v);

    let v: LogEntry = Faker.fake();
    println!("{:#?}", v);

    let v: LogEntryBase32 = Faker.fake();
    println!("{:#?}", v);
}
