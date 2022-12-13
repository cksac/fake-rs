use crate::{Dummy, Fake, Faker};
use rand::Rng;

use random_color::{Luminosity, RandomColor};

impl Dummy<Faker> for RandomColor {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        RandomColor {
            hue: None,
            luminosity: Some(Luminosity::Random),
            seed: Some((u64::MIN..u64::MAX).fake_with_rng::<u64, _>(rng)),
            alpha: Some((0..10).fake_with_rng::<i8, _>(rng) as f32 / 10.),
        }
    }
}
