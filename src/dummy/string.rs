use crate::{any::Any, Dummy};
use rand::distributions::Alphanumeric;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::ops;

const DEFAULT_STR_LEN_RANGE: ops::Range<usize> = 5..20;

impl Dummy<Any> for String {
    fn dummy_ref(_: &Any) -> Self {
        let len = usize::dummy(DEFAULT_STR_LEN_RANGE);
        String::dummy(len)
    }
}

impl Dummy<String> for String {
    fn dummy_ref(t: &String) -> Self {
        t.clone()
    }

    fn dummy(t: String) -> Self {
        t
    }
}

impl Dummy<&str> for String {
    fn dummy_ref(t: &&str) -> Self {
        (**t).to_owned()
    }

    fn dummy(t: &str) -> Self {
        (*t).to_owned()
    }
}

impl Dummy<usize> for String {
    fn dummy_ref(len: &usize) -> Self {
        thread_rng().sample_iter(&Alphanumeric).take(*len).collect()
    }
}

impl Dummy<ops::Range<usize>> for String {
    fn dummy_ref(range: &ops::Range<usize>) -> Self {
        let len = rand::thread_rng().gen_range(range.start, range.end);
        String::dummy(len)
    }
}

impl Dummy<ops::RangeFrom<usize>> for String {
    fn dummy_ref(range: &ops::RangeFrom<usize>) -> Self {
        let u = Uniform::new_inclusive(range.start, std::usize::MAX);
        let len = u.sample(&mut rand::thread_rng());
        String::dummy(len)
    }
}

impl Dummy<ops::RangeFull> for String {
    fn dummy_ref(_: &ops::RangeFull) -> Self {
        let u = Uniform::new_inclusive(std::usize::MIN, std::usize::MAX);
        let len = u.sample(&mut rand::thread_rng());
        String::dummy(len)
    }
}

impl Dummy<ops::RangeInclusive<usize>> for String {
    fn dummy_ref(range: &ops::RangeInclusive<usize>) -> Self {
        let u = Uniform::new_inclusive(range.start(), range.end());
        let len = u.sample(&mut rand::thread_rng());
        String::dummy(len)
    }
}

impl Dummy<ops::RangeTo<usize>> for String {
    fn dummy_ref(range: &ops::RangeTo<usize>) -> Self {
        let len = rand::thread_rng().gen_range(std::usize::MIN, range.end);
        String::dummy(len)
    }
}

impl Dummy<ops::RangeToInclusive<usize>> for String {
    fn dummy_ref(range: &ops::RangeToInclusive<usize>) -> Self {
        let u = Uniform::new_inclusive(std::usize::MIN, range.end);
        let len = u.sample(&mut rand::thread_rng());
        String::dummy(len)
    }
}

impl<'a, L> Dummy<(&'a [u8], L)> for String
where
    usize: Dummy<L>,
{
    fn dummy_ref(config: &(&'a [u8], L)) -> Self {
        let len = usize::dummy_ref(&config.1);
        let s: Option<String> = (0..len)
            .map(|_| Some(*config.0.choose(&mut rand::thread_rng())? as char))
            .collect();
        s.unwrap_or_default()
    }
}

impl<'a> Dummy<&'a [u8]> for String {
    fn dummy_ref(charset: &&[u8]) -> Self {
        String::dummy(*charset)
    }
    fn dummy(charset: &[u8]) -> Self {
        String::dummy((charset, DEFAULT_STR_LEN_RANGE))
    }
}
