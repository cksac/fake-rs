use ::helper::*;
use ::Fake;

pub trait Boolean {
    fn boolean() -> bool;
}

impl<T: Fake> Boolean for T {
    #[inline]
    default fn boolean() -> bool {
        random::<bool>()
    }
}
