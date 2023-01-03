use crate::faker::number::raw::*;
use crate::faker::numerify_sym;
use crate::locales::Data;
use crate::Dummy;
use rand::seq::SliceRandom;
use rand::Rng;

impl<L: Data> Dummy<Digit<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Digit<L>, rng: &mut R) -> Self {
        let s = *L::NUMBER_DIGIT.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<Digit<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Digit<L>, rng: &mut R) -> Self {
        *L::NUMBER_DIGIT.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<NumberWithFormat<'_, L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &NumberWithFormat<L>, rng: &mut R) -> Self {
        numerify_sym(c.1, rng)
    }
}
