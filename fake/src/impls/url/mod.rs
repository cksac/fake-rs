use crate::{Dummy, Fake, Faker};
use rand::seq::SliceRandom;
use rand::Rng;
use url::Url;

const FQDN: &str = "https://example.com";
const PATHS: [&str; 30] = [
    "/apple",
    "/orange",
    "/banana",
    "/grape",
    "/pear",
    "/peach/sweet",
    "/melon/fresh",
    "/kiwi/season",
    "/lemon/acidic",
    "/lime/citrus",
    "/cherry?sweet=true",
    "/berry#mixed",
    "/mango?seasonal=true",
    "/apricot#dried",
    "/avocado?ripe=true",
    "/papaya#exotic",
    "/fig?type=black",
    "/date#sweet",
    "/olive?type=black",
    "/tomato#fresh",
    "/onion?type=red",
    "/carrot#organic",
    "/potato?type=russet",
    "/spinach#fresh",
    "/lettuce?crispy=true",
    "/cabbage#green",
    "/cucumber?seedless=true",
    "/pepper#bell",
    "/garlic?organic=true",
    "/ginger#spicy",
];

impl Dummy<Faker> for Url {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let path: &str = PATHS.choose(rng).unwrap();
        Url::parse(&format!("{FQDN}{path}")).unwrap()
    }
}
