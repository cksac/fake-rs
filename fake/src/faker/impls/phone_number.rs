use crate::faker::numerify_sym;
use crate::faker::phone_number::raw::*;
use crate::locales::Data;
use crate::Dummy;
use rand::seq::IndexedRandom;
use rand::Rng;

impl<L: Data> Dummy<PhoneNumber<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &PhoneNumber<L>, rng: &mut R) -> Self {
        let fmt = L::PHONE_NUMBER_FORMATS.choose(rng).unwrap();
        numerify_sym(fmt, rng)
    }
}

impl<L: Data> Dummy<CellNumber<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_c: &CellNumber<L>, rng: &mut R) -> Self {
        let fmt = L::PHONE_CELL_NUMBER_FORMATS.choose(rng).unwrap();
        numerify_sym(fmt, rng)
    }
}
