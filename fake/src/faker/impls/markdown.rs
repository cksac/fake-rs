use crate::faker::markdown::raw::*;
use crate::faker::lorem::raw::*;
use crate::locales::Data;
use crate::{Dummy, Fake};
use rand::seq::IndexedRandom;
use rand::Rng;

impl<L: Data + Copy> Dummy<ItalicWord<L>> for String {
  fn dummy_with_rng<R: Rng + ?Sized>(_: &ItalicWord<L>, rng: &mut R) -> Self {
      let s = *L::LOREM_WORD.choose(rng).unwrap();
      format!("*{}*", s)
  }
}

impl<L: Data + Copy> Dummy<BoldWord<L>> for String {
  fn dummy_with_rng<R: Rng + ?Sized>(_: &BoldWord<L>, rng: &mut R) -> Self {
      let s = *L::LOREM_WORD.choose(rng).unwrap();
      format!("**{}**", s)
  }
}

impl<L: Data + Copy> Dummy<Link<L>> for String {
  fn dummy_with_rng<R: Rng + ?Sized>(c: &Link<L>, rng: &mut R) -> Self {
      let text: String = Word(c.0).fake_with_rng::<&str, _>(rng).to_string();
      let url: String = Word(c.0).fake_with_rng(rng);
      format!("[{}](https://{})", text, url)
  }
}

impl<L: Data + Copy> Dummy<BulletPoints<L>> for Vec<String> {
  fn dummy_with_rng<R: Rng + ?Sized>(c: &BulletPoints<L>, rng: &mut R) -> Self {
      let len: usize = c.1.fake_with_rng(rng);
      let mut v: Vec<String> = Vec::with_capacity(len);
      let w: Word<L> = Word(c.0);
      for _ in 0..len {
          v.push(format!("- {}", w.fake_with_rng::<&str, _>(rng)));
      }
      v
  }
}

impl<L: Data + Copy> Dummy<ListItems<L>> for Vec<String> {
  fn dummy_with_rng<R: Rng + ?Sized>(c: &ListItems<L>, rng: &mut R) -> Self {
      let len: usize = c.1.fake_with_rng(rng);
      let mut v: Vec<String> = Vec::with_capacity(len);
      let item: Word<L> = Word(c.0);
      for i in 0..len {
          v.push(format!("{}. {}", i + 1, item.fake_with_rng::<&str, _>(rng)));
      }
      v
  }
}

impl<L: Data + Copy> Dummy<BlockQuoteSingleLine<L>> for String {
  fn dummy_with_rng<R: Rng + ?Sized>(c: &BlockQuoteSingleLine<L>, rng: &mut R) -> Self {
      let sentence: String = Sentence(c.0, 4..10).fake_with_rng(rng);
      format!("> {}", sentence)
  }
}

impl<L: Data + Copy> Dummy<BlockQuoteMultiLine<L>> for Vec<String> {
  fn dummy_with_rng<R: Rng + ?Sized>(c: &BlockQuoteMultiLine<L>, rng: &mut R) -> Self {
      let len: usize = c.1.fake_with_rng(rng);
      let mut v: Vec<String> = Vec::with_capacity(len);
      let sentence = Sentence(c.0, 4..10);
      for _ in 0..len {
          v.push(format!("> {}", sentence.fake_with_rng::<String, _>(rng)));
      }
      v
  }
}

impl<L: Data + Copy> Dummy<Code<L>> for String {
  fn dummy_with_rng<R: Rng + ?Sized>(c: &Code<L>, rng: &mut R) -> Self {
      let code: String = Sentence(c.0, 4..10).fake_with_rng::<String, _>(rng).to_string();
      format!("```\n{}\n```", code)
  }
}