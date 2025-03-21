use crate::faker::lorem::raw::*;
use crate::locales::{Data, DynData};
use crate::{Dummy, Fake};
use rand::seq::IndexedRandom;
use rand::Rng;

impl<L: DynData> Dummy<Word<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Word<L>, rng: &mut R) -> Self {
        let s = *c.0.lorem_word().choose(rng).unwrap();
        s.into()
    }
}

impl<L: DynData> Dummy<Word<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Word<L>, rng: &mut R) -> Self {
        c.0.lorem_word().choose(rng).unwrap()
    }
}

impl<L: DynData> Dummy<Words<L>> for Vec<String> {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Words<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let w = Word(c.0.cloned());
        for _ in 0..len {
            v.push(w.fake_with_rng(rng));
        }
        v
    }
}

impl<L: DynData> Dummy<Sentence<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Sentence<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let w = Word(c.0.cloned());
        for _ in 0..len {
            v.push(w.fake_with_rng(rng));
        }
        v.join(" ") + "."
    }
}

impl<L: DynData> Dummy<Sentences<L>> for Vec<String> {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Sentences<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let s = Sentence(c.0.cloned(), 4..10);
        for _ in 0..len {
            v.push(s.fake_with_rng(rng));
        }
        v
    }
}

impl<L: DynData> Dummy<Paragraph<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Paragraph<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let s = Sentence(c.0.cloned(), 4..10);
        for _ in 0..len {
            v.push(s.fake_with_rng(rng));
        }
        v.join("\n")
    }
}

impl<L: DynData> Dummy<Paragraphs<L>> for Vec<String> {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Paragraphs<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let p = Paragraph(c.0.cloned(), 4..7);
        for _ in 0..len {
            v.push(p.fake_with_rng(rng));
        }
        v
    }
}
