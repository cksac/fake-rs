use std::str::FromStr;

use email_address::EmailAddress;
use rand::RngExt;

use crate::{
    faker::internet::raw::{FreeEmail, SafeEmail},
    locales::Data,
    Dummy,
};

impl<L: Data + Copy> Dummy<FreeEmail<L>> for EmailAddress {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &FreeEmail<L>, rng: &mut R) -> Self {
        Self::from_str(&<String as Dummy<FreeEmail<L>>>::dummy_with_rng(c, rng)).unwrap()
    }
}

impl<L: Data + Copy> Dummy<SafeEmail<L>> for EmailAddress {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &SafeEmail<L>, rng: &mut R) -> Self {
        Self::from_str(&<String as Dummy<SafeEmail<L>>>::dummy_with_rng(c, rng)).unwrap()
    }
}
