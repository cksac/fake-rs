use crate::faker::currency::raw::*;
use crate::locales::Data;
use crate::Dummy;
use rand::seq::SliceRandom;
use rand::Rng;

impl<L: Data> Dummy<CurrencyCode<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CurrencyCode<L>, rng: &mut R) -> Self {
        let s = *L::CURRENCY_CODE.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CurrencyCode<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CurrencyCode<L>, rng: &mut R) -> Self {
        *L::CURRENCY_CODE.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<CurrencyName<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CurrencyName<L>, rng: &mut R) -> Self {
        let s = *L::CURRENCY_NAME.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CurrencyName<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CurrencyName<L>, rng: &mut R) -> Self {
        *L::CURRENCY_NAME.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<CurrencySymbol<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CurrencySymbol<L>, rng: &mut R) -> Self {
        let s = *L::CURRENCY_SYMBOL.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CurrencySymbol<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CurrencySymbol<L>, rng: &mut R) -> Self {
        *L::CURRENCY_SYMBOL.choose(rng).unwrap()
    }
}
