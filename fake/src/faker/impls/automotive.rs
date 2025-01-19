use crate::faker::automotive::raw::*;
use crate::locales::FR_FR;
use crate::{Dummy, Fake};
use rand::prelude::SliceRandom;
use rand::Rng;
use std::char;

/* ABC without I, O and U
As with the SIV system, The letters I and O were never used because they could be confused with other characters, like 1 and 0.
ref https://en.wikipedia.org/wiki/Vehicle_registration_plates_of_France

The letter U where also not used because it could be confused with V letter.
ref on french wikipedia article https://fr.wikipedia.org/wiki/Plaque_d%27immatriculation_fran%C3%A7aise
*/
const LICENSE_CHARS: [char; 23] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V',
    'W', 'X', 'Y', 'Z',
];

#[inline]
pub(crate) fn numerify_licence_plate<R: Rng + ?Sized>(string: &str, rng: &mut R) -> String {
    string
        .chars()
        .map(|x| match x {
            '$' => *LICENSE_CHARS.choose(rng).unwrap(),
            '#' => char::from_digit((0..10).fake_with_rng::<u32, _>(rng), 10).unwrap(),
            other => other,
        })
        .collect()
}

const LICENSE_PLATE: &[&str] = &["$$-###-$$"];

impl Dummy<LicencePlate<FR_FR>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &LicencePlate<FR_FR>, rng: &mut R) -> Self {
        let fmt = LICENSE_PLATE.choose(rng).unwrap();
        numerify_licence_plate(fmt, rng)
    }
}
