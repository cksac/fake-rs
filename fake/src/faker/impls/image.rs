use crate::faker::image::raw::*;
use crate::locales::Data;
use crate::{Dummy};
use rand::Rng;

impl<L: Data> Dummy<Unsplash<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Unsplash<L>, rng: &mut R) -> Self {
        format!("https://picsum.photos/{:?}", rng.gen_range(100, 1080))
    }
}

impl<L: Data> Dummy<UnsplashWithSize<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &UnsplashWithSize<L>, _: &mut R) -> Self {
        format!("https://picsum.photos/{:?}/{:?}", c.1, c.2)
    }
}

impl<L: Data> Dummy<UnsplashGrayscale<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &UnsplashGrayscale<L>, _: &mut R) -> Self {
        format!("https://picsum.photos/{:?}/{:?}?grayscale", c.1, c.2)
    }
}

impl<L: Data> Dummy<UnsplashBlur<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &UnsplashBlur<L>, _: &mut R) -> Self {
        format!("https://picsum.photos/{:?}/{:?}?blur={:?}", c.1, c.2, c.3)
    }
}