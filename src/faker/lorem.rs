use ::helper::*;
use ::Fake;
use std::cmp::min;

pub trait Lorem: Fake {
    #[inline]
    fn word() -> &'static str {
        take_one(<Self as Fake>::lorem_word_data())
    }

    #[inline]
    fn words(count: usize) -> Vec<&'static str> {
        let upper = min(<Self as Fake>::lorem_word_data().len() - 1, count);
        let mut result = shuffle(<Self as Fake>::lorem_word_data());
        result.truncate(upper);
        result
    }

    #[inline]
    fn sentence(count: usize, max_extra_count: usize) -> String {
        <Self as Lorem>::words(count + gen_range(0, max_extra_count + 1)).join(" ") + "."
    }

    #[inline]
    fn sentences(count: usize) -> Vec<String> {
        (0..count).map(|_| <Self as Lorem>::sentence(7, 3)).collect()
    }

    #[inline]
    fn paragraph(count: usize, max_extra_count: usize) -> String {
        <Self as Lorem>::sentences(count + gen_range(0, max_extra_count + 1)).join("\n")
    }

    #[inline]
    fn paragraphs(count: usize) -> Vec<String> {
        (0..count).map(|_| <Self as Lorem>::paragraph(7, 3)).collect()
    }
}
