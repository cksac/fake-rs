pub mod address;
pub mod boolean;
pub mod company;
pub mod job;
pub mod name;

use crate::Fake;
use rand::Rng;
use std::char;

#[inline]
fn numerify_sym<R: Rng + ?Sized>(string: &str, rng: &mut R) -> String {
    string
        .chars()
        .map(|x| match x {
            'N' => char::from_digit((1..10).fake_with_rng::<u32, _>(rng), 10).unwrap(),
            '#' => char::from_digit((0..10).fake_with_rng::<u32, _>(rng), 10).unwrap(),
            other => other,
        })
        .collect()
}
