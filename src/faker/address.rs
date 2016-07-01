use ::helper::*;
use ::Fake;
use ::faker::{Name, Number};

pub trait Address {
    fn time_zone() -> &'static str;
    fn city_prefix() -> &'static str;
    fn city_suffix() -> &'static str;
    fn street_suffix() -> &'static str;
    fn state() -> &'static str;
    fn state_abbr() -> &'static str;
    fn city() -> String;
    fn street_name() -> String;
    fn building_number() -> String;
    fn street_address() -> String;
    fn secondary_address() -> String;
    fn zip() -> String;
    fn postcode() -> String;
    fn latitude() -> String;
    fn longitude() -> String;
}

impl<T: Fake> Address for T {
    #[inline]
    default fn time_zone() -> &'static str {
        T::ADDRESS_TIME_ZONE[gen_range(0, T::ADDRESS_TIME_ZONE.len())]
    }

    #[inline]
    default fn city_prefix() -> &'static str {
        T::ADDRESS_CITY_PREFIX[gen_range(0, T::ADDRESS_CITY_PREFIX.len())]
    }

    #[inline]
    default fn city_suffix() -> &'static str {
        T::ADDRESS_CITY_SUFFIX[gen_range(0, T::ADDRESS_CITY_SUFFIX.len())]
    }

    #[inline]
    default fn street_suffix() -> &'static str {
        T::ADDRESS_STREET_SUFFIX[gen_range(0, T::ADDRESS_STREET_SUFFIX.len())]
    }

    #[inline]
    default fn state() -> &'static str {
        T::ADDRESS_STATE[gen_range(0, T::ADDRESS_STATE.len())]
    }

    #[inline]
    default fn state_abbr() -> &'static str {
        T::ADDRESS_STATE_ABBR[gen_range(0, T::ADDRESS_STATE_ABBR.len())]
    }

    #[inline]
    default fn city() -> String {
        match gen_range(0, 5) {
            0 => {
                format!("{} {} {}",
                        <T as Address>::city_prefix(),
                        <T as Name>::first_name(),
                        <T as Address>::city_suffix())
            }
            1 => {
                format!("{} {}",
                        <T as Name>::first_name(),
                        <T as Address>::city_suffix())
            }
            _ => {
                format!("{} {}",
                        <T as Name>::last_name(),
                        <T as Address>::city_suffix())
            }
        }
    }

    #[inline]
    default fn street_name() -> String {
        match gen_range(0, 2) {
            0 => {
                format!("{} {}",
                        <T as Name>::first_name(),
                        <T as Address>::street_suffix())
            }
            _ => {
                format!("{} {}",
                        <T as Name>::last_name(),
                        <T as Address>::street_suffix())
            }
        }
    }

    #[inline]
    default fn building_number() -> String {
        let len = T::ADDRESS_BUILDING_NUMBER_FORMAT.len();
        let number_format = T::ADDRESS_BUILDING_NUMBER_FORMAT[gen_range(0, len)];
        numerify_sym(number_format)
    }

    #[inline]
    default fn street_address() -> String {
        format!("{} {}",
                <T as Address>::building_number(),
                <T as Address>::street_name())
    }

    #[inline]
    default fn secondary_address() -> String {
        format!("{} {}",
                ["Apt.", "Suit."][gen_range(0, 2)],
                <T as Number>::number(3))
    }

    #[inline]
    default fn zip() -> String {
        let len = T::ADDRESS_ZIP_FORMAT.len();
        let number_format = T::ADDRESS_ZIP_FORMAT[gen_range(0, len)];
        numerify_sym(number_format)
    }

    #[inline]
    default fn postcode() -> String {
        let len = T::ADDRESS_POSTCODE_FORMAT.len();
        let number_format = T::ADDRESS_POSTCODE_FORMAT[gen_range(0, len)];
        numerify_sym(number_format)
    }

    #[inline]
    default fn latitude() -> String {
        ((random::<f64>() * 180_f64) - 90_f64).to_string()
    }

    #[inline]
    default fn longitude() -> String {
        ((random::<f64>() * 360_f64) - 180_f64).to_string()
    }
}