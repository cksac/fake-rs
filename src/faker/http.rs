use crate::dummy::{any::Any, Dummy, DummyAny};
use crate::helper;
use crate::http;
use crate::Fake;
use rand::seq::SliceRandom;

const RFC_STATUS_CODES: &[u16] = &[
    100, 101, 102, 200, 201, 202, 203, 204, 205, 206, 207, 208, 226, 300, 301, 302, 303, 304, 305,
    307, 308, 400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416,
    417, 418, 421, 422, 423, 424, 426, 428, 429, 431, 451, 500, 501, 502, 503, 504, 505, 506, 507,
    508, 510, 511,
];

pub trait Http: Fake {
    /// return status code with RFC
    #[inline]
    fn status_code() -> http::StatusCode {
        http::StatusCode::any()
    }

    /// return valid status code within (100, 600]
    #[inline]
    fn all_status_code() -> http::StatusCode {
        let i = helper::gen_range(100, 600);
        http::StatusCode::from_u16(i).unwrap()
    }

    /// return valid status code in given list
    #[inline]
    fn from_status_codes<T: AsRef<[u16]>>(codes: T) -> http::StatusCode {
        http::StatusCode::dummy(codes.as_ref())
    }
}

impl Dummy<Any> for http::StatusCode {
    fn dummy_ref(_: &Any) -> Self {
        let i = helper::gen_range(0, RFC_STATUS_CODES.len());
        http::StatusCode::from_u16(RFC_STATUS_CODES[i]).unwrap()
    }
}

impl Dummy<&[u16]> for http::StatusCode {
    fn dummy_ref(codes: &&[u16]) -> Self {
        let mut r = rand::thread_rng();
        let i = (*codes).choose(&mut r).expect("no codes provided");
        http::StatusCode::from_u16(*i).unwrap()
    }
}
