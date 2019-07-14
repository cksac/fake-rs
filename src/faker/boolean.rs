use crate::locales::Data;
use crate::{Dummy, Fake};
use rand::Rng;

pub struct Boolean<L>(pub L, pub u8);

impl<L: Data> Dummy<Boolean<L>> for bool {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Boolean<L>, rng: &mut R) -> Self {
        let w: u8 = (0..100).fake_with_rng(rng);
        w < c.1
    }
}
