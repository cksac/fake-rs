use crate::locales::Data;
use crate::{Dummy, Fake};
use rand::seq::SliceRandom;
use rand::Rng;

pub struct FirstName<L>(pub L);

impl<L: Data> Dummy<FirstName<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &FirstName<L>, rng: &mut R) -> Self {
        let s = *L::NAME_FIRST_NAME.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<FirstName<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &FirstName<L>, rng: &mut R) -> Self {
        let s = *L::NAME_FIRST_NAME.choose(rng).unwrap();
        s
    }
}

pub struct LastName<L>(pub L);

impl<L: Data> Dummy<LastName<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &LastName<L>, rng: &mut R) -> Self {
        let s = *L::NAME_LAST_NAME.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<LastName<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &LastName<L>, rng: &mut R) -> Self {
        let s = *L::NAME_LAST_NAME.choose(rng).unwrap();
        s
    }
}

pub struct Title<L>(pub L);

impl<L: Data> Dummy<Title<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Title<L>, rng: &mut R) -> Self {
        let s = *L::NAME_TITLE.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<Title<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Title<L>, rng: &mut R) -> Self {
        let s = *L::NAME_TITLE.choose(rng).unwrap();
        s
    }
}

pub struct Suffix<L>(pub L);

impl<L: Data> Dummy<Suffix<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Suffix<L>, rng: &mut R) -> Self {
        let s = *L::NAME_SUFFIX.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<Suffix<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Suffix<L>, rng: &mut R) -> Self {
        let s = *L::NAME_SUFFIX.choose(rng).unwrap();
        s
    }
}

pub struct Name<L>(pub L);

impl<L: Data + Copy> Dummy<Name<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Name<L>, rng: &mut R) -> Self {
        L::NAME_TPL
            .replace("{FirstName}", FirstName(c.0).fake_with_rng::<&str, _>(rng))
            .replace("{LastName}", LastName(c.0).fake_with_rng::<&str, _>(rng))
            .into()
    }
}

pub struct NameWithTitle<L>(pub L);

impl<L: Data + Copy> Dummy<NameWithTitle<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &NameWithTitle<L>, rng: &mut R) -> Self {
        L::NAME_WITH_TITLE_TPL
            .replace("{Title}", Title(c.0).fake_with_rng::<&str, _>(rng))
            .replace("{FirstName}", FirstName(c.0).fake_with_rng::<&str, _>(rng))
            .replace("{LastName}", LastName(c.0).fake_with_rng::<&str, _>(rng))
            .into()
    }
}
