use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Alphanumeric, Distribution, Standard};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::char;
use std::iter;

#[inline]
pub fn gen_range<T: SampleUniform>(start: T, end: T) -> T {
    thread_rng().gen_range(start, end)
}

#[inline]
pub fn random<T>() -> T
where
    Standard: Distribution<T>,
{
    thread_rng().gen()
}

#[inline]
pub fn shuffle<T: Clone>(arr: &[T]) -> Vec<T> {
    let mut v = arr.to_vec();
    let mut rng = thread_rng();
    v.shuffle(&mut rng);
    v
}

#[inline]
pub fn ascii_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect()
}

#[inline]
pub fn gen_vec<T>(length: usize) -> Vec<T>
where
    Standard: Distribution<T>,
{
    let mut rng = thread_rng();
    iter::repeat(()).map(|()| rng.gen()).take(length).collect()
}

#[inline]
pub fn numerify_sym(string: &str) -> String {
    string
        .chars()
        .map(|x| match x {
            'N' => char::from_digit(gen_range(1, 10), 10).unwrap(),
            '#' => char::from_digit(gen_range(0, 10), 10).unwrap(),
            other => other,
        })
        .collect()
}

#[inline]
pub fn take_one<'a, T: ?Sized>(list: &[&'a T]) -> &'a T {
    let mut rng = thread_rng();
    list.choose(&mut rng).expect("take_one got empty list")
}
