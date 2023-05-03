use fake::{Dummy, Fake, Faker, Opt, Optional};

#[derive(Debug, Dummy)]
pub struct Order {
    #[dummy(faker = "0..200")]
    a: Option<u64>,

    #[dummy(faker = "Opt(0..200, 100)", from = "Optional<u64>")]
    always_some: Option<u64>,

    #[dummy(faker = "Opt(0..200, 0)", from = "Optional<u64>")]
    always_none: Option<u64>,

    #[dummy(faker = "0..200")]
    c: Option<Option<u64>>,

    #[dummy(expr = "Opt(Opt(0..200, 50), 50).fake::<Optional<Optional<u64>>>().0.map(|v| v.0)")]
    d: Option<Option<u64>>,
}

fn main() {
    let opt: Optional<usize> = Opt(0..100, 100).fake();
    println!("{:?}", opt);

    let opt: Optional<Optional<usize>> = Opt(Opt(0..200, 50), 20).fake();
    println!("{:?}", opt.0.map(|v| v.0));

    let opt: Option<usize> = Opt(0..100, 100).fake::<Optional<usize>>().into();
    println!("{:?}", opt);

    let o: Order = Faker.fake();
    println!("{:?}", o);
}
