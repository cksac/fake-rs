use crate::locales::{Data, EN};
use crate::{Dummy, Fake, Faker};
use rand::seq::SliceRandom;
use rand::Rng;
use std::path::PathBuf;

static DEFAULT_PATH_FAKER: PathFaker = PathFaker {
    root_dirs: EN::PATH_ROOT_DIRS,
    segments: EN::PATH_SEGMENTS,
    extensions: EN::PATH_EXTENSIONS,
    max_level: 3,
};

impl Dummy<Faker> for PathBuf {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        DEFAULT_PATH_FAKER.fake_with_rng(rng)
    }
}

pub struct PathFaker<'a> {
    root_dirs: &'a [&'a str],
    segments: &'a [&'a str],
    extensions: &'a [&'a str],
    max_level: usize,
}

impl<'a> PathFaker<'a> {
    pub fn new(
        root_dirs: &'a [&'a str],
        segments: &'a [&'a str],
        extensions: &'a [&'a str],
        max_level: usize,
    ) -> Self {
        PathFaker {
            root_dirs,
            segments,
            extensions,
            max_level,
        }
    }
}

impl<'a> Dummy<PathFaker<'a>> for PathBuf {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &PathFaker<'a>, rng: &mut R) -> Self {
        let root_dir = c.root_dirs.choose(rng).unwrap();
        let mut path = PathBuf::from(root_dir);
        for _ in 0..c.max_level {
            if Faker.fake_with_rng::<bool, _>(rng) {
                path.push(c.segments.choose(rng).unwrap());
            }
        }
        if let Some(ext) = c.extensions.choose(rng) {
            path.set_extension(ext);
        }
        path
    }
}
