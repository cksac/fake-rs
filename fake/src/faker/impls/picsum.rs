use crate::faker::picsum::raw::*;
use crate::locales::Data;
use crate::Dummy;
use rand::distr::{Distribution, Uniform};
use rand::Rng;

impl<L: Data> Dummy<Image<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Image<L>, rng: &mut R) -> Self {
        let width: u16 = Uniform::new_inclusive(100, 800)
            .expect("valid range")
            .sample(rng);
        let height: u16 = Uniform::new_inclusive(100, 800)
            .expect("valid range")
            .sample(rng);
        format!("https://picsum.photos/{}/{}", width, height)
    }
}

impl<L: Data> Dummy<ImageWithSeed<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_c: &ImageWithSeed<L>, rng: &mut R) -> Self {
        let width: u16 = Uniform::new_inclusive(100, 800)
            .expect("valid range")
            .sample(rng);
        let height: u16 = Uniform::new_inclusive(100, 800)
            .expect("valid range")
            .sample(rng);
        let seed: u32 = rng.random();
        format!("https://picsum.photos/seed/{}/{}/{}", seed, width, height)
    }
}

impl<L: Data> Dummy<ImageGrayscale<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &ImageGrayscale<L>, rng: &mut R) -> Self {
        let width: u16 = Uniform::new_inclusive(100, 800)
            .expect("valid range")
            .sample(rng);
        let height: u16 = Uniform::new_inclusive(100, 800)
            .expect("valid range")
            .sample(rng);
        format!("https://picsum.photos/{}/{}?grayscale", width, height)
    }
}

impl<L: Data> Dummy<ImageBlur<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &ImageBlur<L>, rng: &mut R) -> Self {
        let width: u16 = Uniform::new_inclusive(100, 800)
            .expect("valid range")
            .sample(rng);
        let height: u16 = Uniform::new_inclusive(100, 800)
            .expect("valid range")
            .sample(rng);
        let blur: u8 = Uniform::new_inclusive(1, 10)
            .expect("valid range")
            .sample(rng);
        format!("https://picsum.photos/{}/{}?blur={}", width, height, blur)
    }
}

impl<L: Data> Dummy<ImageCustom<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &ImageCustom<L>, rng: &mut R) -> Self {
        let width: u16 = c.1.width.unwrap_or_else(|| {
            Uniform::new_inclusive(100, 800)
                .expect("valid range")
                .sample(rng)
        });
        let height: u16 = c.1.height.unwrap_or_else(|| {
            Uniform::new_inclusive(100, 800)
                .expect("valid range")
                .sample(rng)
        });

        let mut url = format!("https://picsum.photos/{}/{}", width, height);
        let mut params = Vec::new();

        if c.1.grayscale {
            params.push("grayscale".to_string());
        }

        if let Some(blur) = c.1.blur {
            params.push(format!("blur={}", blur));
        }

        if let Some(ref seed) = c.1.seed {
            url = format!("https://picsum.photos/seed/{}/{}/{}", seed, width, height);
        }

        if !params.is_empty() {
            url.push_str("?");
            url.push_str(&params.join("&"));
        }

        url
    }
}

/// Configuration options for custom Picsum images
#[derive(Debug, Clone, Default)]
pub struct ImageOptions {
    pub width: Option<u16>,
    pub height: Option<u16>,
    pub grayscale: bool,
    pub blur: Option<u8>,
    pub seed: Option<String>,
}

impl ImageOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }

    pub fn grayscale(mut self) -> Self {
        self.grayscale = true;
        self
    }

    pub fn blur(mut self, blur: u8) -> Self {
        self.blur = Some(blur);
        self
    }

    pub fn seed(mut self, seed: impl Into<String>) -> Self {
        self.seed = Some(seed.into());
        self
    }
}
