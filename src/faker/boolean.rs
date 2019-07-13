use crate::locales::Data;
use crate::{Dummy, Fake};
use rand::Rng;

pub struct Boolean(pub u8);

impl Dummy<Boolean> for bool {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Boolean, rng: &mut R) -> Self {
        let w: u8 = (0..100).fake_with_rng(rng);
        w < c.0
    }
}