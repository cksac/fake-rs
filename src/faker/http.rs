use helper;
use http;
use Dummy;
use Fake;

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
        http::StatusCode::dummy()
    }

    /// return valid status code within (100, 600]
    #[inline]
    fn all_status_code() -> http::StatusCode {
        let i = helper::gen_range(100, 600);
        http::StatusCode::from_u16(i).unwrap()
    }
}

impl Dummy for http::StatusCode {
    fn dummy() -> Self {
        let i = helper::gen_range(0, RFC_STATUS_CODES.len());
        http::StatusCode::from_u16(RFC_STATUS_CODES[i]).unwrap()
    }
}
