use crate::faker::http::raw::*;
use crate::locales::Data;
use crate::{Dummy, Fake, Faker};
use rand::RngExt;

impl<L: Data> Dummy<RfcStatusCode<L>> for http::StatusCode {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &RfcStatusCode<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng(rng)
    }
}

impl<L: Data> Dummy<RfcStatusCode<L>> for String {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &RfcStatusCode<L>, rng: &mut R) -> Self {
        let code: http::StatusCode = Faker.fake_with_rng(rng);
        format!("{}", code)
    }
}

impl<L: Data> Dummy<ValidStatusCode<L>> for http::StatusCode {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &ValidStatusCode<L>, rng: &mut R) -> Self {
        let code: u16 = (100..600).fake_with_rng(rng);
        http::StatusCode::from_u16(code).unwrap()
    }
}

impl<L: Data> Dummy<ValidStatusCode<L>> for String {
    #[inline]
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &ValidStatusCode<L>, rng: &mut R) -> Self {
        let code: u16 = (100..600).fake_with_rng(rng);
        format!("{}", http::StatusCode::from_u16(code).unwrap())
    }
}
