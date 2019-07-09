use crate::helper::*;
use crate::Fake;

pub trait Boolean: Fake {
    #[inline]
    fn boolean() -> bool {
        random::<bool>()
    }
}
