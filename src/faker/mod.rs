pub mod address;
pub mod boolean;
pub mod company;
pub mod internet;
pub mod job;
pub mod lorem;
pub mod name;
pub mod number;
pub mod phone_number;

use crate::Fake;
use rand::Rng;
use std::char;

#[inline]
fn numerify_sym<R: Rng + ?Sized>(string: &str, rng: &mut R) -> String {
    string
        .chars()
        .map(|x| match x {
            '^' => char::from_digit((1..10).fake_with_rng::<u32, _>(rng), 10).unwrap(),
            '#' => char::from_digit((0..10).fake_with_rng::<u32, _>(rng), 10).unwrap(),
            other => other,
        })
        .collect()
}
