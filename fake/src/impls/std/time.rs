use std::time::Duration;

use crate::{Dummy, Fake, Faker};
use rand::Rng;

impl Dummy<Faker> for Duration {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Duration::from_nanos(Faker.fake_with_rng(rng))
    }
}
