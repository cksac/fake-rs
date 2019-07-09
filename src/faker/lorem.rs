use crate::helper::*;
use std::cmp::min;
use crate::Fake;

pub trait Lorem: Fake {
    #[inline]
    fn word() -> &'static str {
        take_one(Self::lorem_word_data())
    }

    #[inline]
    fn words(count: usize) -> Vec<&'static str> {
        let upper = min(Self::lorem_word_data().len() - 1, count);
        let mut result = shuffle(Self::lorem_word_data());
        result.truncate(upper);
        result
    }

    #[inline]
    fn sentence(count: usize, max_extra_count: usize) -> String {
        Self::words(count + gen_range(0, max_extra_count + 1)).join(" ") + "."
    }

    #[inline]
    fn sentences(count: usize) -> Vec<String> {
        (0..count).map(|_| Self::sentence(7, 3)).collect()
    }

    #[inline]
    fn paragraph(count: usize, max_extra_count: usize) -> String {
        Self::sentences(count + gen_range(0, max_extra_count + 1)).join("\n")
    }

    #[inline]
    fn paragraphs(count: usize) -> Vec<String> {
        (0..count).map(|_| Self::paragraph(7, 3)).collect()
    }
}
