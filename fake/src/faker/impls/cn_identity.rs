//! Chinese identity document fakers: national ID (GB 11643-1999 checksum) and
//! bank card numbers (Luhn). Locale-agnostic data generators — the locale
//! parameter exists only to fit the crate's `def_fakers!` machinery.

use crate::faker::cn_identity::raw::{BankCardNumber, IdentityCardNumber};
use crate::locales::Data;
use rand::SeedableRng;
use crate::{Dummy, Fake};
use rand::RngExt;
use rand::seq::IndexedRandom;

/// Real first-two-digit region codes (province level, GB/T 2260).
pub const REGION_PREFIXES: &[&str] = &[
    "11", "12", "13", "14", "15", "21", "22", "23", "31", "32", "33", "34", "35", "36", "37", "41",
    "42", "43", "44", "45", "46", "50", "51", "52", "53", "54", "61", "62", "63", "64", "65",
];

/// GB 11643-1999 checksum weights (first 17 digits).
pub const ID_WEIGHTS: [u32; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];

/// Check-code lookup: index = sum of weighted digits mod 11.
pub const ID_CHECK_CODES: &[char] = &['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];

/// Append a Luhn check digit to a digit body.
pub fn luhn_check_digit(body: &str) -> char {
    let sum: u32 = body
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let v = (c as u32) - ('0' as u32);
            if i % 2 == 0 {
                let d = v * 2;
                if d > 9 {
                    d - 9
                } else {
                    d
                }
            } else {
                v
            }
        })
        .sum();
    char::from_digit((10 - sum % 10) % 10, 10).unwrap()
}

/// Standard Luhn validity check (used by tests and downstream verification).
pub fn luhn_valid(card: &str) -> bool {
    if card.len() < 12 || card.chars().any(|c| !c.is_ascii_digit()) {
        return false;
    }
    let sum: u32 = card
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let v = (c as u32) - ('0' as u32);
            if i % 2 == 1 {
                let d = v * 2;
                if d > 9 {
                    d - 9
                } else {
                    d
                }
            } else {
                v
            }
        })
        .sum();
    sum % 10 == 0
}

/// Independent GB 11643-1999 checksum verification (test helper).
pub fn id_checksum_valid(id: &str) -> bool {
    if id.len() != 18 {
        return false;
    }
    let body: Vec<char> = id.chars().take(17).collect();
    if body.iter().any(|c| !c.is_ascii_digit()) {
        return false;
    }
    let sum: u32 = body
        .iter()
        .zip(ID_WEIGHTS.iter())
        .map(|(c, w)| ((*c as u32) - ('0' as u32)) * w)
        .sum();
    id.chars().last().unwrap() == ID_CHECK_CODES[(sum % 11) as usize]
}

/// Bank card BIN prefixes with their conventional total lengths.
pub const BANK_BINS: &[(u32, usize)] = &[
    (415185, 16),
    (452992, 16),
    (513342, 16),
    (532894, 16),
    (621697, 17),
    (622126, 19),
    (622588, 19),
    (622848, 19),
];

fn random_birth<R: RngExt + ?Sized>(rng: &mut R) -> (u32, u32, u32) {
    let y: u32 = (1950..2010).fake_with_rng(rng);
    let m: u32 = (1..=12).fake_with_rng(rng);
    let d: u32 = (1..=28).fake_with_rng(rng);
    (y, m, d)
}

impl<L: Data> Dummy<IdentityCardNumber<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &IdentityCardNumber<L>, rng: &mut R) -> String {
        let region = format!(
            "{}{:02}{:02}",
            REGION_PREFIXES.choose(rng).unwrap(),
            (0..=18).fake_with_rng::<u32, _>(rng),
            (0..=18).fake_with_rng::<u32, _>(rng)
        );
        let (y, m, d) = random_birth(rng);
        let body: String = format!(
            "{}{:04}{:02}{:02}{:03}",
            region,
            y,
            m,
            d,
            (0..=999).fake_with_rng::<u32, _>(rng)
        );
        let mut s = body;
        let sum: u32 = s
            .chars()
            .zip(ID_WEIGHTS.iter())
            .map(|(c, w)| ((c as u32) - ('0' as u32)) * w)
            .sum();
        s.push(ID_CHECK_CODES[(sum % 11) as usize]);
        s
    }
}

impl<L: Data> Dummy<BankCardNumber<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &BankCardNumber<L>, rng: &mut R) -> String {
        let (bin, default_len) = *BANK_BINS.choose(rng).unwrap();
        let len = if c.1 == 0 {
            default_len
        } else {
            c.1.clamp(12, 19)
        };
        let mut s = bin.to_string();
        while s.len() < len - 1 {
            s.push(char::from_digit((0..10).fake_with_rng::<u32, _>(rng), 10).unwrap());
        }
        format!("{}{}", s, luhn_check_digit(&s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Fake;
    use crate::locales::EN;

    #[test]
    fn id_checksums_hold() {
        let mut rng = rand::rngs::StdRng::from_seed([3u8; 32]);
        for _ in 0..500 {
            let id: String = IdentityCardNumber(EN).fake_with_rng(&mut rng);
            assert_eq!(id.len(), 18);
            assert!(id_checksum_valid(&id), "bad checksum: {id}");
            assert!(REGION_PREFIXES.contains(&&id[..2]), "bad region: {id}");
            let y: u32 = id[6..10].parse().unwrap();
            assert!((1950..2010).contains(&y), "bad year: {id}");
        }
    }

    #[test]
    fn bank_cards_pass_luhn() {
        let mut rng = rand::rngs::StdRng::from_seed([4u8; 32]);
        for _ in 0..500 {
            let card: String = BankCardNumber(EN, 0).fake_with_rng(&mut rng);
            assert!((12..=19).contains(&card.len()), "bad len: {card}");
            assert!(luhn_valid(&card), "bad luhn: {card}");
        }
    }

    #[test]
    fn bank_card_explicit_length() {
        let mut rng = rand::rngs::StdRng::from_seed([5u8; 32]);
        for _ in 0..50 {
            let card: String = BankCardNumber(EN, 16).fake_with_rng(&mut rng);
            assert_eq!(card.len(), 16);
            assert!(luhn_valid(&card));
        }
    }
}
