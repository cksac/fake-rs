use crate::{Dummy, Fake, Faker};
use chrono_tz::{Tz, TZ_VARIANTS};
use rand::RngExt;

impl Dummy<Faker> for Tz {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let index: usize = (0..TZ_VARIANTS.len()).fake_with_rng(rng);
        TZ_VARIANTS[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_tz() {
        let tz: Tz = Faker.fake();
        assert!(TZ_VARIANTS.contains(&tz));
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn dummy_tz_through_datetime() {
        let datetime: chrono::DateTime<Tz> = Faker.fake();
        assert!(TZ_VARIANTS.contains(&datetime.timezone()));
    }
}
