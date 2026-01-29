use fake::{Dummy, Fake, Faker, Opt, Optional};

#[derive(Debug, Dummy)]
pub struct Order {
    #[dummy(faker = "0..200")]
    pub a: Option<u64>,

    #[dummy(faker = "Opt(0..200, 100)", from = "Optional<u64>")]
    pub always_some: Option<u64>,

    #[dummy(expr = "Some((0..200).fake())")]
    pub always_some_v2: Option<u64>,

    #[dummy(faker = "Opt(0..200, 0)", from = "Optional<u64>")]
    pub always_none: Option<u64>,

    #[dummy(expr = "None")]
    pub always_none_v2: Option<u64>,

    #[dummy(faker = "0..200")]
    pub c: Option<Option<u64>>,

    #[dummy(expr = "Opt(Opt(0..200, 50), 50).fake::<Optional<Optional<u64>>>().0.map(|v| v.0)")]
    pub d: Option<Option<u64>>,
}

fn main() {
    let opt: Optional<usize> = Opt(0..100, 100).fake();
    println!("{opt:?}");

    let opt: Optional<Optional<usize>> = Opt(Opt(0..200, 50), 20).fake();
    println!("{:?}", opt.0.map(|v| v.0));

    let opt: Option<usize> = Opt(0..100, 100).fake::<Optional<usize>>().into();
    println!("{opt:?}");

    let o: Order = Faker.fake();
    println!("{o:?}");
}
