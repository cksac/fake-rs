use crate::faker::lorem::raw::*;
use crate::locales::Data;
use crate::{Dummy, Fake};
use rand::seq::IndexedRandom;
use rand::RngExt;

impl<L: Data> Dummy<Word<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Word<L>, rng: &mut R) -> Self {
        let s = *L::LOREM_WORD.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<Word<L>> for &str {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &Word<L>, rng: &mut R) -> Self {
        L::LOREM_WORD.choose(rng).unwrap()
    }
}

impl<L: Data + Copy> Dummy<Words<L>> for Vec<String> {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &Words<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let w = Word(c.0);
        for _ in 0..len {
            v.push(w.fake_with_rng(rng));
        }
        v
    }
}

impl<L: Data + Copy> Dummy<Sentence<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &Sentence<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let w = Word(c.0);
        for _ in 0..len {
            v.push(w.fake_with_rng(rng));
        }
        v.join(" ") + "."
    }
}

impl<L: Data + Copy> Dummy<Sentences<L>> for Vec<String> {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &Sentences<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let s = Sentence(c.0, 4..10);
        for _ in 0..len {
            v.push(s.fake_with_rng(rng));
        }
        v
    }
}

impl<L: Data + Copy> Dummy<Paragraph<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &Paragraph<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let s = Sentence(c.0, 4..10);
        for _ in 0..len {
            v.push(s.fake_with_rng(rng));
        }
        v.join("\n")
    }
}

impl<L: Data + Copy> Dummy<Paragraphs<L>> for Vec<String> {
    fn dummy_with_rng<R: RngExt + ?Sized>(c: &Paragraphs<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let p = Paragraph(c.0, 4..7);
        for _ in 0..len {
            v.push(p.fake_with_rng(rng));
        }
        v
    }
}
