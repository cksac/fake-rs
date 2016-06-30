use ::helper::*;
use ::Fake;
use rand::distributions::range::SampleRange;

pub trait Number {
    fn digit() -> String;
    fn number(count: usize) -> String;
    fn between<T: PartialOrd + SampleRange>(start: T, end: T) -> T;
}

impl<T: Fake> Number for T {
    #[inline]
    default fn digit() -> String {
        gen_range(0, 9).to_string()
    }

    #[inline]
    default fn number(count: usize) -> String {
        (0..count).map(|_| <T as Number>::digit()).collect::<Vec<String>>().join("")
    }

    #[inline]
    default fn between<U: PartialOrd + SampleRange>(start: U, end: U) -> U {
        gen_range(start, end)
    }
}
