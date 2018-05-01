use helper::*;
use Fake;

pub trait PhoneNumber: Fake {
    #[inline]
    fn phone_number() -> String {
        let number_format = take_one(Self::phone_number_format_data());
        numerify_sym(number_format)
    }

    #[inline]
    fn phone_number_with_format<S: AsRef<str>>(format: S) -> String {
        numerify_sym(format.as_ref())
    }

    #[inline]
    fn cell_number() -> String {
        let number_format = take_one(Self::phone_cell_number_format_data());
        numerify_sym(number_format)
    }
}
