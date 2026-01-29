use crate::faker::finance::raw::*;
use crate::locales::Data;
use crate::{Dummy, Fake};
use rand::seq::IndexedRandom;
use rand::Rng;

const ALPHABET: &[char; 26] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const ALPHANUMERIC: &[char; 36] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

const ISO3166: &[&str] = &[
    "AC", "AD", "AE", "AF", "AG", "AI", "AL", "AM", "AN", "AO", "AQ", "AR", "AS", "AT", "AU", "AW",
    "AX", "AZ", "BA", "BB", "BD", "BE", "BF", "BG", "BH", "BI", "BJ", "BL", "BM", "BN", "BO", "BQ",
    "BR", "BS", "BT", "BU", "BV", "BW", "BY", "BZ", "CA", "CC", "CD", "CE", "CF", "CG", "CH", "CI",
    "CK", "CL", "CM", "CN", "CO", "CP", "CR", "CS", "CS", "CU", "CV", "CW", "CX", "CY", "CZ", "DD",
    "DE", "DG", "DJ", "DK", "DM", "DO", "DZ", "EA", "EC", "EE", "EG", "EH", "ER", "ES", "ET", "EU",
    "FI", "FJ", "FK", "FM", "FO", "FR", "FX", "GA", "GB", "GD", "GE", "GF", "GG", "GH", "GI", "GL",
    "GM", "GN", "GP", "GQ", "GR", "GS", "GT", "GU", "GW", "GY", "HK", "HM", "HN", "HR", "HT", "HU",
    "IC", "ID", "IE", "IL", "IM", "IN", "IO", "IQ", "IR", "IS", "IT", "JE", "JM", "JO", "JP", "KE",
    "KG", "KH", "KI", "KM", "KN", "KP", "KR", "KW", "KY", "KZ", "LA", "LB", "LC", "LI", "LK", "LR",
    "LS", "LT", "LU", "LV", "LY", "MA", "MC", "MD", "ME", "MF", "MG", "MH", "MK", "ML", "MM", "MN",
    "MO", "MP", "MQ", "MR", "MS", "MT", "MU", "MV", "MW", "MX", "MY", "MZ", "NA", "NC", "NE", "NF",
    "NG", "NI", "NL", "NO", "NP", "NR", "NT", "NU", "NZ", "OM", "PA", "PE", "PF", "PG", "PH", "PK",
    "PL", "PM", "PN", "PR", "PS", "PT", "PW", "PY", "QA", "RE", "RO", "RS", "RU", "RW", "SA", "SB",
    "SC", "SD", "SE", "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SR", "SS", "ST", "SU",
    "SV", "SX", "SY", "SZ", "TA", "TC", "TD", "TF", "TG", "TH", "TJ", "TK", "TL", "TM", "TN", "TO",
    "TR", "TT", "TV", "TW", "TZ", "UA", "UG", "UM", "US", "UY", "UZ", "VA", "VC", "VE", "VG", "VI",
    "VN", "VU", "WF", "WS", "YE", "YT", "YU", "ZA", "ZM", "ZR", "ZW",
];

const VOWELS: &[char; 5] = &['A', 'E', 'I', 'O', 'U'];

impl<L: Data> Dummy<Bic<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Bic<L>, rng: &mut R) -> Self {
        let prob: i8 = (0..100).fake_with_rng(rng);
        let suffix = if prob < 50 {
            return format!(
                "{}{}{}{}{}{}1",
                *ALPHABET.choose(rng).unwrap(),
                *ALPHABET.choose(rng).unwrap(),
                *ALPHABET.choose(rng).unwrap(),
                *VOWELS.choose(rng).unwrap(),
                *ISO3166.choose(rng).unwrap(),
                *ALPHABET.choose(rng).unwrap(),
            );
        } else if prob < 90 {
            (
                rng.random_range('0'..='9'),
                rng.random_range('0'..='9'),
                rng.random_range('0'..='9'),
            )
        } else {
            (
                *ALPHABET.choose(rng).unwrap(),
                *VOWELS.choose(rng).unwrap(),
                *ALPHABET.choose(rng).unwrap(),
            )
        };
        format!(
            "{}{}{}{}{}{}1{}{}{}",
            *ALPHABET.choose(rng).unwrap(),
            *ALPHABET.choose(rng).unwrap(),
            *ALPHABET.choose(rng).unwrap(),
            *VOWELS.choose(rng).unwrap(),
            *ISO3166.choose(rng).unwrap(),
            *ALPHABET.choose(rng).unwrap(),
            suffix.0,
            suffix.1,
            suffix.2,
        )
    }
}

fn split_number_to_digits(x: u32) -> Vec<u32> {
    x.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

impl<L: Data> Dummy<Isin<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Isin<L>, rng: &mut R) -> Self {
        let country_code = *ISO3166.choose(rng).unwrap();
        let nsin = (1..10)
            .map(|_x| *ALPHANUMERIC.choose(rng).unwrap())
            .collect::<String>();
        // Checksum calculation according to Luhn algorithm
        let char_to_num: Vec<u32> = country_code
            .chars()
            .chain(nsin.chars())
            .map(|x| {
                if x.is_ascii_digit() {
                    x.to_digit(10).unwrap()
                } else {
                    10 + ((x as u32) - ('A' as u32))
                }
            })
            .flat_map(split_number_to_digits)
            .collect();
        let checksum_even = char_to_num.iter().rev().skip(1).step_by(2).sum::<u32>();
        let checksum_odd = char_to_num
            .iter()
            .rev()
            .step_by(2)
            .map(|&x| x * 2)
            .flat_map(split_number_to_digits)
            .sum::<u32>();
        let checksum_digit = (10 - ((checksum_even + checksum_odd) % 10)) % 10;
        format!("{country_code}{nsin}{checksum_digit}")
    }
}
