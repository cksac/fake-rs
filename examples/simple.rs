#[macro_use]
extern crate fake;

fn main() {
    println!("{:?}", fake!(Lorem.word));
    println!("{:?}", fake!(Number.number(10)));
    println!("{:?}", fake!(Lorem.sentence(4, 6)));
    println!("{}", fake!(Name.name in zh_tw));
}
