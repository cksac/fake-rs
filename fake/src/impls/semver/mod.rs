use crate::faker::boolean::raw::Boolean;
use crate::locales::EN;
use crate::{Dummy, Fake, Faker};
use rand::seq::SliceRandom;
use rand::Rng;

const UNSTABLE_SEMVER: &[&str] = &["alpha", "beta", "rc"];

impl Dummy<Faker> for semver::Version {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let probability = 10;
        let pre = if Boolean(EN, probability).fake_with_rng(rng) {
            semver::Prerelease::new(&format!(
                "{}.{}",
                *UNSTABLE_SEMVER.choose(rng).unwrap(),
                &(0..9).fake_with_rng::<u8, _>(rng).to_string()
            ))
            .unwrap()
        } else {
            semver::Prerelease::EMPTY
        };
        semver::Version {
            major: (0..9).fake_with_rng::<u64, _>(rng),
            minor: (0..20).fake_with_rng::<u64, _>(rng),
            patch: (0..20).fake_with_rng::<u64, _>(rng),
            pre,
            build: semver::BuildMetadata::EMPTY,
        }
    }
}
