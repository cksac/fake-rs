use crate::faker::administrative::raw::*;
use crate::locales::FR_FR;
use crate::{Dummy, Fake};
use rand::seq::IndexedRandom;
use rand::Rng;

// ref https://fr.wikipedia.org/wiki/Num%C3%A9rotation_des_d%C3%A9partements_fran%C3%A7ais
const FR_FR_DEPARTMENTS: &[&str] = &[
    "01", "02", "03", "04", "05", "06", "07", "08", "09", "11", "12", "13", "14", "15", "16", "17",
    "18", "19", "2A", "2B", "21", "22", "23", "24", "25", "26", "27", "28", "29", "31", "32", "33",
    "34", "35", "36", "37", "38", "39", "41", "42", "43", "44", "45", "46", "47", "48", "49", "51",
    "52", "53", "54", "55", "56", "57", "58", "59", "61", "62", "63", "64", "65", "66", "67", "68",
    "69", "71", "72", "73", "74", "75", "76", "77", "78", "79", "81", "82", "83", "84", "85", "86",
    "87", "88", "89", "91", "92", "93", "94", "95", "971", "972", "973", "974", "975", "976",
    "977", "978", "984", "986", "987", "988", "989",
];

impl Dummy<HealthInsuranceCode<FR_FR>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &HealthInsuranceCode<FR_FR>, rng: &mut R) -> Self {
        // ref https://www.previssima.fr/lexique/numero-de-securite-sociale-a-13-chiffres.html
        // and test on http://marlot.org/util/calcul-de-la-cle-nir.php
        let sex: u8 = (1..3).fake_with_rng::<u8, _>(rng);
        let birth_year: u8 = (0..99).fake_with_rng::<u8, _>(rng);
        let birth_month: u8 = (1..13).fake_with_rng::<u8, _>(rng);
        let department: &str = FR_FR_DEPARTMENTS.choose(rng).unwrap();
        let town_code: u16 = (0..999).fake_with_rng::<u16, _>(rng);
        let order_code: u16 = (0..999).fake_with_rng::<u16, _>(rng);
        let department_code: u16 = match department {
            "2A" => 19,
            "2B" => 18,
            _ => department.parse::<u16>().unwrap(),
        };
        let number = format!(
            "{sex}{birth_year:02}{birth_month:02}{department_code}{town_code:03}{order_code:03}"
        )
        .parse::<u64>()
        .unwrap();
        let key = 97 - (number % 97);
        format!(
            "{sex} {birth_year:02} {birth_month:02} {department} {town_code:03} {order_code:03} {key}"
        )
    }
}
