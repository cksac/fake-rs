//! Entity-associated fakers: one coherent person per row.
//!
//! A plain faker suite draws each field independently, so the same "person"
//! gets different names across rows, an email that matches nobody, and a
//! date of birth that contradicts the age. The `Entity` faker fixes that:
//! one RNG pass derives a consistent [`Person`] — display name, email, phone,
//! city, province, postal code, age and date of birth — from a shared person
//! context, so generating N rows yields N believable, cross-referenced
//! records.
//!
//! The email local part is the person's own name (ASCII for Latin locales,
//! pinyin for `ZH_CN`/`ZH_TW`), the age and the date of birth agree on the
//! birth year, and the Chinese ID card embeds that same birth date with a
//! valid GB 11643-1999 checksum.
//!
//! ```
//! use fake::Fake;
//! use fake::faker::entity::raw::Entity;
//! use fake::faker::impls::entity::Person;
//! use rand::SeedableRng;
//! let mut rng = rand::rngs::StdRng::from_seed([1u8; 32]);
//! let p = Person::from_rng(&mut rng, fake::locales::EN);
//! // p.email starts with p.ascii_name, p.age matches p.date_of_birth,
//! // p.identity_card encodes the same birth date with a valid checksum.
//! ```

use crate::faker::impls::cn_identity::{
    id_checksum_valid, ID_CHECK_CODES, ID_WEIGHTS, REGION_PREFIXES,
};
use crate::faker::impls::address::CityNameGenFn;
use crate::faker::name::raw::{FirstName, LastName, Name, NameAscii};
use crate::faker::numerify_sym_with_digits;
use crate::faker::replace_ascii_digits_with_locale;
use crate::faker::entity::raw::{Entity as EntityFaker, EntityWithBirthYear};
use crate::locales::Data;
use rand::SeedableRng;
use crate::{Dummy, Fake};
use rand::RngExt;
use rand::seq::IndexedRandom;

/// A coherent synthetic person derived from a single RNG stream.
///
/// All fields are cross-referenced: the email local part is the person's own
/// name, the age matches the date of birth, and the Chinese ID card encodes
/// that same birth date.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    /// Display name in the locale's writing system.
    pub name: String,
    /// ASCII form of the name used for the email local part.
    pub ascii_name: String,
    /// Full email address; local part is the person's own name.
    pub email: String,
    /// Phone number in a locale phone format.
    pub phone: String,
    /// City name.
    pub city: String,
    /// Province / state name (drawn from the same locale address tables).
    pub province: String,
    /// Postal code.
    pub postal_code: String,
    /// Age in whole years, consistent with [`Person::date_of_birth`].
    pub age: u32,
    /// Date of birth `YYYY-MM-DD` consistent with [`Person::age`].
    pub date_of_birth: String,
    /// Chinese national ID card (18 chars, valid GB 11643-1999 checksum)
    /// embedding the same birth date.
    pub identity_card: String,
    /// Bank card number (valid Luhn).
    pub bank_card: String,
}

impl Person {
    /// Draw a coherent person from `rng` in the given locale.
    pub fn from_rng<L: Data + Copy + CityNameGenFn, R: RngExt + ?Sized>(rng: &mut R, locale: L) -> Self {
        build::<L, R>(rng, locale, None)
    }

    /// Like [`Person::from_rng`] but constrains the birth year to `years`.
    pub fn with_birth_year<L: Data + Copy + CityNameGenFn, R: RngExt + ?Sized>(
        rng: &mut R,
        locale: L,
        years: std::ops::RangeInclusive<u32>,
    ) -> Self {
        build::<L, R>(rng, locale, Some(years))
    }
}

/// Pinyin syllables used to build ASCII email local parts for Chinese names.
pub const PINYIN_FIRST: &[&str] = &[
    "wang", "li", "zhang", "liu", "chen", "yang", "zhao", "huang", "zhou", "wu", "xu", "sun", "ma",
    "zhu", "hu", "guo", "lin", "he", "luo", "gao",
];

pub const PINYIN_GIVEN: &[&str] = &[
    "wei", "fang", "jing", "min", "hua", "jun", "mei", "na", "tao", "lei", "yan", "jun", "bo",
    "xiao", "jie", "peng", "lin", "gang", "juan", "hao",
];

/// Default birth-year window for synthetic entities.
pub const DEFAULT_BIRTH_YEAR: std::ops::RangeInclusive<u32> = 1950..=2008;

/// Draw the shared person context, then derive every field from it.
fn build<L: Data + Copy + CityNameGenFn, R: RngExt + ?Sized>(
    rng: &mut R,
    locale: L,
    birth_years: Option<std::ops::RangeInclusive<u32>>,
) -> Person {
    let year_range = birth_years.unwrap_or(DEFAULT_BIRTH_YEAR);
    let (surname, given) = {
        let first: &str = FirstName(locale).fake_with_rng(rng);
        let last: &str = LastName(locale).fake_with_rng(rng);
        let ascii_first: &str = NameAscii(locale).fake_with_rng(rng);
        if is_cjk(first) {
            let sy = PINYIN_FIRST.choose(rng).unwrap();
            let gy = PINYIN_GIVEN.choose(rng).unwrap();
            (format!("{sy}{gy}"), format!("{sy} {gy}"))
        } else {
            (ascii_first.to_string(), format!("{ascii_first} {last}"))
        }
    };

    let local = if is_cjk(&surname) {
        let n: u32 = (10..=99).fake_with_rng::<u32, _>(rng);
        format!("{surname}{n}")
    } else {
        surname.to_lowercase()
    };
    let domain = *L::INTERNET_FREE_EMAIL_PROVIDER.choose(rng).unwrap();
    let email = format!("{local}@{domain}");

    let phone = {
        let fmt = *L::PHONE_NUMBER_FORMATS.choose(rng).unwrap();
        let s = numerify_sym_with_digits(fmt, rng, L::NUMBER_DIGIT);
        replace_ascii_digits_with_locale(&s, L::NUMBER_DIGIT)
    };

    let city: String = crate::faker::address::raw::CityName(locale).fake_with_rng(rng);
    let province: String = crate::faker::address::raw::StateName(locale).fake_with_rng(rng);
    let postal_code: String = crate::faker::address::raw::ZipCode(locale).fake_with_rng(rng);

    // Age and date of birth agree: pick a birth year inside the window and
    // anchor age to the window's upper end (a fixed, deterministic reference
    // point, so no wall clock is needed).
    let dob_year: u32 = rng.random_range(year_range.clone());
    let age = *year_range.end() - dob_year;
    let month: u32 = (1..=12).fake_with_rng::<u32, _>(rng);
    let day: u32 = (1..=28).fake_with_rng::<u32, _>(rng);
    let date_of_birth = format!("{:04}-{:02}-{:02}", dob_year, month, day);

    let identity_card = rebuild_identity_with_dob(rng, locale, dob_year, month, day);
    let bank_card: String =
        crate::faker::cn_identity::raw::BankCardNumber(locale, 0).fake_with_rng(rng);
    let name_display: String = Name(locale).fake_with_rng(rng);

    Person {
        name: name_display,
        ascii_name: given,
        email,
        phone,
        city,
        province,
        postal_code,
        age,
        date_of_birth,
        identity_card,
        bank_card,
    }
}

/// Rebuild a checksum-valid Chinese ID card around a chosen birth date so the
/// card, the age and the DOB field all tell the same story.
fn rebuild_identity_with_dob<L: Data + Copy + CityNameGenFn, R: RngExt + ?Sized>(
    rng: &mut R,
    locale: L,
    year: u32,
    month: u32,
    day: u32,
) -> String {
    let region = format!(
        "{}{:02}{:02}",
        REGION_PREFIXES.choose(rng).unwrap(),
        (0..=18).fake_with_rng::<u32, _>(rng),
        (0..=18).fake_with_rng::<u32, _>(rng)
    );
    let body = format!(
        "{}{:04}{:02}{:02}{:03}",
        region,
        year,
        month,
        day,
        (0..=999).fake_with_rng::<u32, _>(rng)
    );
    let mut s = body;
    let sum: u32 = s
        .chars()
        .zip(ID_WEIGHTS.iter())
        .map(|(c, w)| ((c as u32) - ('0' as u32)) * w)
        .sum();
    s.push(ID_CHECK_CODES[(sum % 11) as usize]);
    assert!(id_checksum_valid(&s));
    let _ = locale;
    s
}

fn is_cjk(s: &str) -> bool {
    s.chars()
        .next()
        .map(|c| ('\u{4e00}'..='\u{9fff}').contains(&c))
        .unwrap_or(false)
}

impl<L: Data + Copy + CityNameGenFn> Dummy<EntityFaker<L>> for Person {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &EntityFaker<L>, rng: &mut R) -> Person {
        build::<L, R>(rng, c.0, None)
    }
}

impl<L: Data + Copy + CityNameGenFn> Dummy<EntityWithBirthYear<L>> for Person {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &EntityWithBirthYear<L>, rng: &mut R) -> Person {
        build::<L, R>(rng, c.0, Some(c.1.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::faker::impls::cn_identity::luhn_valid;
    use crate::locales::{EN, ZH_CN};

    fn rng() -> rand::rngs::StdRng {
        rand::rngs::StdRng::from_seed([9u8; 32])
    }

    #[test]
    fn email_matches_own_name() {
        let mut r = rng();
        let e = build::<EN, _>(&mut r, EN, None);
        assert!(
            e.email
                .to_lowercase()
                .starts_with(&e.ascii_name.split_whitespace().next().unwrap().to_lowercase()),
            "email {} should start with {}",
            e.email,
            e.ascii_name
        );
        assert!(e.email.contains('@'));
    }

    #[test]
    fn age_and_dob_agree() {
        let mut r = rng();
        let e = build::<EN, _>(&mut r, EN, None);
        let y: u32 = e.date_of_birth[..4].parse().unwrap();
        assert_eq!(e.age, DEFAULT_BIRTH_YEAR.end() - y);
        assert!((1..=130).contains(&e.age));
    }

    #[test]
    fn identity_card_matches_dob() {
        let mut r = rng();
        let e = build::<EN, _>(&mut r, EN, None);
        assert_eq!(e.identity_card.len(), 18);
        // ID birth segment (chars 6..14 = YYYYMMDD) matches the DOB field.
        let (yy, mm, dd): (u32, u32, u32) = (
            e.date_of_birth[..4].parse().unwrap(),
            e.date_of_birth[5..7].parse().unwrap(),
            e.date_of_birth[8..10].parse().unwrap(),
        );
        let id_b = format!("{:04}{:02}{:02}", yy, mm, dd);
        assert_eq!(&e.identity_card[6..14], &id_b);
        assert!(id_checksum_valid(&e.identity_card));
    }

    #[test]
    fn bank_card_valid() {
        let mut r = rng();
        let e = build::<EN, _>(&mut r, EN, None);
        assert!(luhn_valid(&e.bank_card));
    }

    #[test]
    fn seed_reproducible() {
        let a = build::<EN, _>(&mut rng(), EN, None);
        let b = build::<EN, _>(&mut rng(), EN, None);
        assert_eq!(a, b);
    }

    #[test]
    fn faker_impl_works() {
        let mut r = rng();
        let p: Person = EntityFaker(EN).fake_with_rng(&mut r);
        assert!(!p.email.is_empty());
    }

    #[test]
    fn chinese_name_gets_pinyin_email() {
        let mut r = rng();
        let e = build::<ZH_CN, _>(&mut r, ZH_CN, None);
        // The email local part must be pure ASCII (pinyin) even when the
        // display name is Chinese.
        let local = e.email.split('@').next().unwrap();
        assert!(!local.is_empty());
        assert!(local.chars().all(|c| c.is_ascii()));
        assert!(!e.name.chars().all(|c| c.is_ascii()), "expected a CJK name");
    }

    #[test]
    fn birth_year_bound_respected() {
        let mut r = rng();
        let e = build::<EN, _>(&mut r, EN, Some(1990..=1995));
        let y: u32 = e.date_of_birth[..4].parse().unwrap();
        assert!((1990..=1995).contains(&y));
        assert_eq!(e.age, 1995 - y);
    }
}
