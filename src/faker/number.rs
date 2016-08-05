use ::helper::*;
use ::Fake;
use rand::distributions::range::SampleRange;

pub trait Number: Fake {
    #[inline]
    fn digit() -> String {
        gen_range(0_u8, 10_u8).to_string()
    }

    #[inline]
    fn number(count: usize) -> String {
        (0..count).map(|_| Self::digit()).collect::<Vec<_>>().concat()
    }

    #[inline]
    fn between<U: PartialOrd + SampleRange>(start: U, end: U) -> U {
        gen_range(start, end)
    }
}
