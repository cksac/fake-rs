use crate::{Dummy, Fake, Faker};
use crate::faker::boolean::raw::Boolean;
use crate::locales::{EN};
use rand::seq::SliceRandom;
use rand::Rng;

pub const UNSTABLE_SEMVER: &'static [&'static str] = &[
    "alpha", "beta", "rc"
];

impl Dummy<Faker> for semver::Version {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let probability = 10;
        let pre;
        if Boolean(EN, probability).fake_with_rng(rng) {
            pre = vec!(semver::Identifier::AlphaNumeric(format!(
                "{}.{}",
                *UNSTABLE_SEMVER.choose(rng).unwrap(),
                &(0..9).fake_with_rng::<u8, _>(rng).to_string()
            )));
        } else {
            pre = vec!();
        }
        semver::Version {
            major: (0..9).fake_with_rng::<u64, _>(rng),
            minor: (0..20).fake_with_rng::<u64, _>(rng),
            patch: (0..20).fake_with_rng::<u64, _>(rng),
            pre: pre,
            build: vec!(),
        }
    }
}
