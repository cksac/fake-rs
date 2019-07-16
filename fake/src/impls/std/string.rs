use crate::{Dummy, Fake, Faker};
use rand::distributions::Alphanumeric;
use rand::seq::SliceRandom;
use rand::Rng;
use std::ops;

const DEFAULT_STR_LEN_RANGE: ops::Range<usize> = 5..20;

impl Dummy<usize> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(len: &usize, rng: &mut R) -> Self {
        rng.sample_iter(&Alphanumeric).take(*len).collect()
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
