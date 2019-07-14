use crate::locales::Data;
use crate::{Dummy, Fake};
use rand::seq::SliceRandom;
use rand::Rng;
use std::ops;

pub struct Word<L>(pub L);

impl<L: Data> Dummy<Word<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Word<L>, rng: &mut R) -> Self {
        let s = *L::LOREM_WORD.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<Word<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Word<L>, rng: &mut R) -> Self {
        let s = *L::LOREM_WORD.choose(rng).unwrap();
        s
    }
}

pub struct Words<L>(pub L, pub ops::Range<usize>);

impl<L: Data + Copy> Dummy<Words<L>> for Vec<String> {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Words<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let w = Word(c.0);
        for _ in 0..len {
            v.push(w.fake_with_rng(rng));
        }
        v
    }
}

pub struct Sentence<L>(pub L, pub ops::Range<usize>);

impl<L: Data + Copy> Dummy<Sentence<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Sentence<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let w = Word(c.0);
        for _ in 0..len {
            v.push(w.fake_with_rng(rng));
        }
        v.join(" ") + "."
    }
}

pub struct Sentences<L>(pub L, pub ops::Range<usize>);

impl<L: Data + Copy> Dummy<Sentences<L>> for Vec<String> {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Sentences<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let s = Sentence(c.0, 4..10);
        for _ in 0..len {
            v.push(s.fake_with_rng(rng));
        }
        v
    }
}

pub struct Paragraph<L>(pub L, pub ops::Range<usize>);

impl<L: Data + Copy> Dummy<Paragraph<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Paragraph<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let s = Sentence(c.0, 4..10);
        for _ in 0..len {
            v.push(s.fake_with_rng(rng));
        }
        v.join("\n")
    }
}

pub struct Paragraphs<L>(pub L, pub ops::Range<usize>);

impl<L: Data + Copy> Dummy<Paragraphs<L>> for Vec<String> {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Paragraphs<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let mut v: Vec<String> = Vec::with_capacity(len);
        let p = Paragraph(c.0, 4..7);
        for _ in 0..len {
            v.push(p.fake_with_rng(rng));
        }
        v
    }
}
