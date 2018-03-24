use Fake;
use helper::*;

pub trait Boolean: Fake {
    #[inline]
    fn boolean() -> bool {
        random::<bool>()
    }
}
