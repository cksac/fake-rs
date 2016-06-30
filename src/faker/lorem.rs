use ::helper::*;
use ::Fake;
use std::cmp::min;

pub trait Lorem {
    fn word() -> &'static str;
    fn words(count: usize) -> Vec<&'static str>;
    fn sentence(count: usize, max_extra_count: usize) -> String;
    fn sentences(count: usize) -> Vec<String>;
    fn paragraph(count: usize, max_extra_count: usize) -> String;
    fn paragraphs(count: usize) -> Vec<String>;
}

impl<T: Fake> Lorem for T {
    #[inline]
    default fn word() -> &'static str {
        T::LOREM_WORD[gen_range(0, T::LOREM_WORD.len())]
    }

    #[inline]
    default fn words(count: usize) -> Vec<&'static str> {
        let upper = min(T::LOREM_WORD.len(), count);
        shuffle(T::LOREM_WORD)[0..upper].to_vec()
    }

    #[inline]
    default fn sentence(count: usize, max_extra_count: usize) -> String {
        <T as Lorem>::words(count + gen_range(0, max_extra_count)).join(" ") + "."
    }

    #[inline]
    default fn sentences(count: usize) -> Vec<String> {
        let mut vec = Vec::with_capacity(count);
        for _ in 0..count {
            vec.push(<T as Lorem>::sentence(7, 3));
        }
        vec
    }

    #[inline]
    default  fn paragraph(count: usize, max_extra_count: usize) -> String {
        <T as Lorem>::sentences(count + gen_range(0, max_extra_count)).join("\n")
    }

    #[inline]
    default fn paragraphs(count: usize) -> Vec<String> {
        let mut vec = Vec::with_capacity(count);
        for _ in 0..count {
            vec.push(<T as Lorem>::paragraph(7, 3));
        }
        vec
    }
}
