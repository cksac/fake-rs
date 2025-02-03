use fake::{Fake, Faker};
use rand::distr;
use rand::rngs::StdRng;
use rand::SeedableRng;

fn main() {
    // generate random u8
    println!("u8 ({}) in [MIN, MAX)", Faker.fake::<u8>());

    // generate random u8 using range
    println!("u8 ({}) in [3,7)", (3..7).fake::<u8>());
    println!("u8 ({}) in [3,7]", (3..=7).fake::<u8>());
    println!("u8 ({}) in [3, MAX]", (3..).fake::<u8>());
    println!("u8 ({}) in [MIN, 7)", (..7).fake::<u8>());
    println!("u8 ({}) in [MIN, 7]", (..=7).fake::<u8>());
    println!("u8 ({}) in [MIN, MAX]", (..).fake::<u8>());

    // to reuse sampler `Uniform` for value generation
    let sampler = distr::Uniform::new_inclusive(1, 10).expect("Can");
    for _ in 0..5 {
        let v: usize = sampler.fake();
        println!("sample value {}", v);
    }

    // fixed seed rng
    let seed = [
        1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let rng = &mut StdRng::from_seed(seed);
    println!("{:?}", Faker.fake_with_rng::<u8, _>(rng));
    println!("{:?}", (1..8).fake_with_rng::<u8, _>(rng));
    for _ in 0..5 {
        let v: usize = sampler.fake_with_rng(rng);
        println!("sample value from fixed seed {}", v);
    }
}
