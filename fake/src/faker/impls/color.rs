use crate::faker::color::raw::*;
use crate::locales::Data;
use crate::{Dummy, Fake, Faker};
use rand::Rng;

impl<L: Data> Dummy<HexColor<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &HexColor<L>, rng: &mut R) -> Self {
        let color: random_color::RandomColor = Faker.fake_with_rng(rng);
        color.to_hex()
    }
}

impl<L: Data> Dummy<RgbColor<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &RgbColor<L>, rng: &mut R) -> Self {
        let color: random_color::RandomColor = Faker.fake_with_rng(rng);
        color.to_rgb_string()
    }
}

impl<L: Data> Dummy<RgbaColor<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &RgbaColor<L>, rng: &mut R) -> Self {
        let color: random_color::RandomColor = Faker.fake_with_rng(rng);
        color.to_rgba_string()
    }
}

impl<L: Data> Dummy<HslColor<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &HslColor<L>, rng: &mut R) -> Self {
        let color: random_color::RandomColor = Faker.fake_with_rng(rng);
        color.to_hsl_string()
    }
}

impl<L: Data> Dummy<HslaColor<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &HslaColor<L>, rng: &mut R) -> Self {
        let color: random_color::RandomColor = Faker.fake_with_rng(rng);
        color.to_hsla_string()
    }
}

impl<L: Data> Dummy<Color<L>> for String {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Color<L>, rng: &mut R) -> Self {
        let _color: random_color::RandomColor = Faker.fake_with_rng(rng);
        format!(
            "{}\n{}\n{}\n{}\n{}",
            _color.to_hex(),
            _color.to_rgb_string(),
            _color.to_rgba_string(),
            _color.to_hsl_string(),
            _color.to_hsla_string()
        )
    }
}
