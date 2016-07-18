use ::helper::*;
use ::Fake;

pub trait PhoneNumber {
    fn phone_number() -> String;
    fn phone_number_with_format<S: AsRef<str>>(format: S) -> String;
    fn cell_number() -> String;
}

impl<T: Fake> PhoneNumber for T {
    #[inline]
    default fn phone_number() -> String {
        let len = T::PHONE_NUMBER_FORMAT.len();
        let number_format = T::PHONE_NUMBER_FORMAT[gen_range(0, len)];
        numerify_sym(number_format)
    }

    #[inline]
    default fn phone_number_with_format<S: AsRef<str>>(format: S) -> String {
        numerify_sym(format.as_ref())
    }

    #[inline]
    default fn cell_number() -> String {
        let len = T::PHONE_CELL_NUMBER_FORMAT.len();
        let number_format = T::PHONE_CELL_NUMBER_FORMAT[gen_range(0, len)];
        numerify_sym(number_format)
    }
}
