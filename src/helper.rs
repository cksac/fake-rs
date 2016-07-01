use rand::{thread_rng, Rng, Rand};
use rand::distributions::range::SampleRange;

#[inline]
pub fn gen_range<T: PartialOrd + SampleRange>(start: T, end: T) -> T {
    thread_rng().gen_range(start, end)
}

#[inline]
pub fn random<T: Rand>() -> T {
    thread_rng().gen()
}

#[inline]
pub fn shuffle<T: Clone>(arr: &[T]) -> Vec<T> {
    let mut v = arr.to_vec();
    thread_rng().shuffle(&mut *v);
    v
}

#[inline]
pub fn ascii_string(length: usize) -> String {
    thread_rng().gen_ascii_chars().take(length).collect()
}

#[inline]
pub fn gen_vec<T: Rand>(length: usize) -> Vec<T> {
    thread_rng().gen_iter::<T>().take(length).collect::<Vec<T>>()
}

#[inline]
pub fn numerify_sym(string: &str) -> String {
        string.chars().map(|x| match x {
            'N' => gen_range(1_i32, 10_i32).to_string(),
            '#' => gen_range(0_i32, 10_i32).to_string(),
            other => other.to_string()
        }).collect::<Vec<String>>().join("")
}