use crate::{Dummy, Fake, Faker};
use rand::Rng;
use url::Url;

const FQDN: &str = "https://example.com";

impl Dummy<Faker> for Url {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let path: String = Faker.fake_with_rng(rng);
        Url::parse(&format!("{FQDN}/{path}")).unwrap()
    }
}
