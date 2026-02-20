use crate::faker::job::raw::*;
use crate::locales::Data;
use crate::{Dummy, Fake};
use rand::seq::IndexedRandom;
use rand::RngExt;

impl<L: Data> Dummy<Seniority<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Seniority<L>, rng: &mut R) -> Self {
        let s = *L::JOB_SENIORITY.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<Seniority<L>> for &str {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Seniority<L>, rng: &mut R) -> Self {
        L::JOB_SENIORITY.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<Field<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Field<L>, rng: &mut R) -> Self {
        let s = *L::JOB_FIELD.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<Field<L>> for &str {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Field<L>, rng: &mut R) -> Self {
        L::JOB_FIELD.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<Position<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Position<L>, rng: &mut R) -> Self {
        let s = *L::JOB_POSITION.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<Position<L>> for &str {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Position<L>, rng: &mut R) -> Self {
        L::JOB_POSITION.choose(rng).unwrap()
    }
}

impl<L: Data + Copy> Dummy<Title<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &Title<L>, rng: &mut R) -> Self {
        L::JOB_TITLE_TPL
            .replace("{Seniority}", Seniority(c.0).fake_with_rng::<&str, _>(rng))
            .replace("{Field}", Field(c.0).fake_with_rng::<&str, _>(rng))
            .replace("{Position}", Position(c.0).fake_with_rng::<&str, _>(rng))
    }
}
