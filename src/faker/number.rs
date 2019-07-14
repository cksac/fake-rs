use super::numerify_sym;
use crate::locales::Data;
use crate::Dummy;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct Digit<L>(pub L);

impl<L: Data> Dummy<Digit<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Digit<L>, rng: &mut R) -> Self {
        let s = *L::NUMBER_DIGIT.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<Digit<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Digit<L>, rng: &mut R) -> Self {
        let s = *L::NUMBER_DIGIT.choose(rng).unwrap();
        s
    }
}

pub struct NumberWithFormat<L>(pub L, pub &'static str);

impl<L: Data> Dummy<NumberWithFormat<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &NumberWithFormat<L>, rng: &mut R) -> Self {
        numerify_sym(c.1, rng)
    }
}
