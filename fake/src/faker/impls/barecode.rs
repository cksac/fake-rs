use crate::faker::barecode::raw::*;
use crate::faker::boolean::raw::Boolean;
use crate::faker::numerify_sym;
use crate::locales::Data;
use crate::{Dummy, Fake};
use rand::Rng;
use rand::prelude::SliceRandom;

const ISBN_MAX_LENGTH:usize = 13;

struct IsbnProperties {
    ean: &'static str,
    group: &'static str,
    registrant: String,
    publication: String
}

impl IsbnProperties {
    fn to_string(&self, sum10: bool) -> String {
        let mut ean = self.ean;
        if sum10 {
            ean = "";
        }
        format!(
            "{}{}{}{}",
            ean,
            self.group,
            self.registrant,
            self.publication
        )
    }
}

fn checksum10(properties: &IsbnProperties) -> String {
    let all = properties.to_string(true);
    let chars: Vec<char> = all.chars().collect();

    let mut sum = 0;
    for (i, v) in chars.iter().enumerate() {
        if let Ok(_v) = v.to_string().parse::<i32>() {
            sum += (i + 1) * _v as usize;
        }
    }

    let check_digit = sum % 11;
    if check_digit == 10 {
        return "X".to_string();
    }
    check_digit.to_string()
}

fn checksum13(properties: &IsbnProperties) -> i32 {
    let all = properties.to_string(false);
    let chars: Vec<char> = all.chars().collect();

    let mut sum = 0;
    for (i, v) in chars.iter().enumerate() {
        if let Ok(_v) = v.to_string().parse::<i32>() {
            if i % 2 == 0 {
                sum += _v;
            } else {
                sum += _v * 3;
            }
        }
    }

    let remainder = sum % 10;
    let mut check_digit = 10 - remainder;
    if check_digit == 10 {
        check_digit = 0;
    }
    check_digit
}

fn get_properties<L: Data, R: Rng + ?Sized>(_c: L, rng: &mut R) -> IsbnProperties {
    let ean = L::ISBN_EAN;
    let rules = *L::isbn_rules();
    let keys:Vec<&'static str> = rules.keys().cloned().collect();
    let group = keys.choose(rng).unwrap();
    
    let reg_pub_len = ISBN_MAX_LENGTH - ean.chars().count() - group.chars().count() - 1;
    let reg_pub = numerify_sym(&std::iter::repeat("#").take(reg_pub_len).collect::<String>(), rng);
    
    let mut reg_len = 0;
    let sufix_reg_pub = &reg_pub[..reg_pub_len as usize - 1];

    for r in &rules[group] {
        if r.min <= sufix_reg_pub && sufix_reg_pub <= r.max {
            reg_len = r.registrant_len;
            break;
        }
    }
    IsbnProperties {
        ean: ean,
        group: group,
        registrant:  reg_pub[..reg_len as usize].to_string(),
        publication: reg_pub[reg_len as usize..reg_pub_len as usize].to_string()
    }
}

impl<L: Data + Copy> Dummy<Isbn10<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Isbn10<L>, rng: &mut R) -> Self {
        let properties = get_properties(c.0, rng);
        format!(
            "{}-{}-{}-{}",
            properties.group,
            properties.registrant,
            properties.publication,
            checksum10(&properties)
        )
    }
}

impl<L: Data + Copy> Dummy<Isbn13<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Isbn13<L>, rng: &mut R) -> Self {
        let properties = get_properties(c.0, rng);
        format!(
            "{}-{}-{}-{}-{}",
            properties.ean,
            properties.group,
            properties.registrant,
            properties.publication,
            checksum13(&properties)
        )
    }
}

impl<L: Data + Copy> Dummy<Isbn<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Isbn<L>, rng: &mut R) -> Self {
        if Boolean(c.0, 50).fake_with_rng(rng) {
            return Isbn13(c.0).fake_with_rng(rng);
        }
        Isbn10(c.0).fake_with_rng(rng)
    }
}
