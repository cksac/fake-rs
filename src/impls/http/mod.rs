use crate::{Dummy, Fake, Faker};
use rand::seq::SliceRandom;
use rand::Rng;

const RFC_STATUS_CODES: &[u16] = &[
    100, 101, 102, 200, 201, 202, 203, 204, 205, 206, 207, 208, 226, 300, 301, 302, 303, 304, 305,
    307, 308, 400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416,
    417, 418, 421, 422, 423, 424, 426, 428, 429, 431, 451, 500, 501, 502, 503, 504, 505, 506, 507,
    508, 510, 511,
];

impl Dummy<Faker> for http::StatusCode {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let code = RFC_STATUS_CODES.choose(rng).unwrap();
        http::StatusCode::from_u16(*code).unwrap()
    }
}

impl Dummy<&[u16]> for Result<http::StatusCode, http::status::InvalidStatusCode> {
    fn dummy_with_rng<R: Rng + ?Sized>(codes: &&[u16], rng: &mut R) -> Self {
        let code = codes.choose(rng).expect("no codes provided");
        http::StatusCode::from_u16(*code)
    }
}

impl Dummy<&[u16]> for http::StatusCode {
    fn dummy_with_rng<R: Rng + ?Sized>(codes: &&[u16], rng: &mut R) -> Self {
        let code = codes.choose(rng).expect("no codes provided");
        http::StatusCode::from_u16(*code).expect("invalid status code")
    }
}

impl Dummy<Faker> for http::Version {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let i: u8 = (0..4).fake_with_rng(rng);
        match i {
            0 => http::Version::HTTP_2,
            1 => http::Version::HTTP_10,
            2 => http::Version::HTTP_09,
            _ => http::Version::HTTP_11,
        }
    }
}
