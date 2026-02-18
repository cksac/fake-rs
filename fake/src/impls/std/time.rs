use std::time::Duration;

use crate::{Dummy, Fake, Faker};
use rand::RngExt;

impl Dummy<Faker> for Duration {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Duration::from_nanos(Faker.fake_with_rng(rng))
    }
}
