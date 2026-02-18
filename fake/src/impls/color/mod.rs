use crate::{Dummy, Fake, Faker};
use rand::RngExt;

use random_color::{options::Luminosity, RandomColor};

impl Dummy<Faker> for RandomColor {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let mut c = RandomColor::new();
        c.seed(Faker.fake_with_rng::<u64, _>(rng))
            .alpha(Faker.fake_with_rng::<f32, _>(rng))
            .luminosity(Luminosity::Random);
        c
    }
}
