use crate::{Dummy, Fake, Faker};
use http::uri;
use rand::seq::SliceRandom;
use rand::Rng;
use std::net::Ipv4Addr;
use std::str::FromStr;

const RFC_STATUS_CODES: &[u16] = &[
    100, 101, 102, 200, 201, 202, 203, 204, 205, 206, 207, 208, 226, 300, 301, 302, 303, 304, 305,
    307, 308, 400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416,
    417, 418, 421, 422, 423, 424, 426, 428, 429, 431, 451, 500, 501, 502, 503, 504, 505, 506, 507,
    508, 510, 511,
];

const VALID_SCHEME_CHARACTERS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '.', '-', '+',
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

impl Dummy<Faker> for http::Uri {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &Faker, rng: &mut R) -> Self {
        // Some common schemes or a random valid scheme
        let scheme = match (0..7).fake_with_rng(rng) {
            0 => "http".to_string(),
            1 => "https".to_string(),
            2 => "ws".to_string(),
            3 => "wss".to_string(),
            4 => "ftp".to_string(),
            5 => "git".to_string(),
            _ => {
                // A valid scheme is any letter followed by any combination of letters, digits, '+',
                // '.', '-'. Looking at a list of know schemes 28 seems to be the longest so I'll
                // generate one a max of that long.

                let len = rng.gen_range(1..29);
                let mut scheme = String::with_capacity(len);
                scheme.push(rng.gen_range(b'a'..=b'z') as char);
                if rng.gen_bool(0.5) {
                    scheme.make_ascii_uppercase();
                }
                scheme.extend((1..len).map(|_| *VALID_SCHEME_CHARACTERS.choose(rng).unwrap()));

                scheme
            }
        };

        let mut authority = String::new();

        if rng.gen_bool(0.5) {
            // Include password
            let user = config.fake_with_rng::<String, _>(rng);
            let username = url_escape::encode_userinfo(&user);
            authority.push_str(&username);
            if rng.gen_bool(0.5) {
                authority.push(':');
                let pass = config.fake_with_rng::<String, _>(rng);
                let password = url_escape::encode_userinfo(&pass);
                authority.push_str(&password);
            }
            authority.push('@');
        }

        let host_type = (0..3).fake_with_rng(rng);
        match host_type {
            0 => {
                authority.push_str("localhost");
                // Include port number
                if rng.gen_bool(0.5) {
                    let port_num = config.fake_with_rng::<u16, _>(rng);
                    authority.push(':');
                    authority.push_str(&port_num.to_string());
                }
            }
            1 => {
                // ip
                let ip: Ipv4Addr = config.fake_with_rng(rng);
                authority.push_str(&ip.to_string());
            }
            _ => {
                if rng.gen_bool(0.5) {
                    authority.push_str("www.");
                }
                let host_len = (1..100).fake_with_rng(rng);
                authority
                    .extend((0..host_len).map(|_| *VALID_SCHEME_CHARACTERS.choose(rng).unwrap()));

                // Include port number
                if rng.gen_bool(0.5) {
                    let port_num = config.fake_with_rng::<u16, _>(rng);
                    authority.push(':');
                    authority.push_str(&port_num.to_string());
                }
            }
        }

        let mut path = format!(
            "/{}",
            url_escape::encode_path(&config.fake_with_rng::<String, _>(rng))
        );

        if rng.gen_bool(0.5) {
            path.push('?');
            let mut query_parts = vec![];
            let query_len = (2..5).fake_with_rng(rng);
            for _ in 1..query_len {
                let key = url_escape::encode_component(&config.fake_with_rng::<String, _>(rng))
                    .to_string();
                let value = url_escape::encode_component(&config.fake_with_rng::<String, _>(rng))
                    .to_string();
                query_parts.push(format!("{}={}", key, value));
            }
            path.push_str(&query_parts.join("&"));
        }

        uri::Builder::new()
            .scheme(uri::Scheme::from_str(&scheme).unwrap())
            .authority(authority)
            .path_and_query(path)
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_urls_generated() {
        for _ in 0..1000 {
            let _url = Faker.fake::<http::Uri>();
            println!("{}", _url);
        }
    }
}
