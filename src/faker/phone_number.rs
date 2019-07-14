use super::numerify_sym;
use crate::locales::Data;
use crate::Dummy;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct PhoneNumber<L>(pub L);

impl<L: Data> Dummy<PhoneNumber<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &PhoneNumber<L>, rng: &mut R) -> Self {
        let fmt = L::PHONE_NUMBER_FORMATS.choose(rng).unwrap();
        numerify_sym(fmt, rng)
    }
}

pub struct CellNumber<L>(pub L);

impl<L: Data> Dummy<CellNumber<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &CellNumber<L>, rng: &mut R) -> Self {
        let fmt = L::PHONE_CELL_NUMBER_FORMATS.choose(rng).unwrap();
        numerify_sym(fmt, rng)
    }
}
