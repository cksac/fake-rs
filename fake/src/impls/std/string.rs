use crate::{Dummy, Fake, Faker};
use rand::distr::Alphanumeric;
use rand::seq::IndexedRandom;
use rand::Rng;
use std::ops;

const DEFAULT_STR_LEN_RANGE: ops::Range<usize> = 5..20;

impl Dummy<usize> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(len: &usize, rng: &mut R) -> Self {
        rng.sample_iter(&Alphanumeric)
            .take(*len)
            .map(char::from)
            .collect()
    }
}

impl Dummy<Faker> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let len: usize = DEFAULT_STR_LEN_RANGE.fake_with_rng(rng);
        len.fake_with_rng(rng)
    }
}

impl Dummy<ops::Range<usize>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::Range<usize>, rng: &mut R) -> Self {
        let len: usize = range.fake_with_rng(rng);
        len.fake_with_rng(rng)
    }
}

impl Dummy<ops::RangeFrom<usize>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeFrom<usize>, rng: &mut R) -> Self {
        let len: usize = range.fake_with_rng(rng);
        len.fake_with_rng(rng)
    }
}

impl Dummy<ops::RangeFull> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeFull, rng: &mut R) -> Self {
        let len: usize = range.fake_with_rng(rng);
        len.fake_with_rng(rng)
    }
}

impl Dummy<ops::RangeInclusive<usize>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeInclusive<usize>, rng: &mut R) -> Self {
        let len: usize = range.fake_with_rng(rng);
        len.fake_with_rng(rng)
    }
}

impl Dummy<ops::RangeTo<usize>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeTo<usize>, rng: &mut R) -> Self {
        let len: usize = range.fake_with_rng(rng);
        len.fake_with_rng(rng)
    }
}

impl Dummy<ops::RangeToInclusive<usize>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(range: &ops::RangeToInclusive<usize>, rng: &mut R) -> Self {
        let len: usize = range.fake_with_rng(rng);
        len.fake_with_rng(rng)
    }
}

/// Custom fake [`String`] generator.
///
/// # Examples
///
/// ```
/// use fake::{Fake, StringFaker};
/// use fake::faker::name::en::Name;
///
/// // weak password generator
/// const ASCII: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&\'()*+,-./:;<=>?@";
/// let f = StringFaker::with(Vec::from(ASCII), 8..12);
/// let a: String = f.fake();
/// ```
pub struct StringFaker<L> {
    charset: Vec<u8>,
    len: L,
}

impl StringFaker<Faker> {
    pub fn charset(charset: Vec<u8>) -> Self {
        StringFaker {
            charset,
            len: Faker,
        }
    }
}

impl<L> StringFaker<L> {
    pub fn with(charset: Vec<u8>, len: L) -> Self {
        StringFaker { charset, len }
    }
}

impl<L> Dummy<StringFaker<L>> for String
where
    usize: Dummy<L>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &StringFaker<L>, rng: &mut R) -> Self {
        let len: usize = config.len.fake_with_rng(rng);
        let s: Option<String> = (0..len)
            .map(|_| Some(*config.charset.choose(rng)? as char))
            .collect();
        s.unwrap_or_default()
    }
}
