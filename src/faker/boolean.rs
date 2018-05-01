use helper::*;
use Fake;

pub trait Boolean: Fake {
    #[inline]
    fn boolean() -> bool {
        random::<bool>()
    }
}
