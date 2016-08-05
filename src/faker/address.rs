use ::helper::*;
use ::Fake;
use ::faker::{Name, Number};

pub trait Address: Fake + Name + Number {
    #[inline]
    fn time_zone() -> &'static str {
        take_one(<Self as Fake>::address_time_zone_data())
    }

    #[inline]
    fn city_prefix() -> &'static str {
        take_one(<Self as Fake>::address_city_prefix_data())
    }

    #[inline]
    fn city_suffix() -> &'static str {
        take_one(<Self as Fake>::address_city_suffix_data())
    }

    #[inline]
    fn street_suffix() -> &'static str {
        take_one(<Self as Fake>::address_street_suffix_data())
    }

    #[inline]
    fn state() -> &'static str {
        take_one(<Self as Fake>::address_state_data())
    }

    #[inline]
    fn state_abbr() -> &'static str {
        take_one(<Self as Fake>::address_state_abbr_data())
    }

    #[inline]
    fn city() -> String {
        match gen_range(0, 5) {
            0 => {
                format!("{} {} {}",
                        <Self as Address>::city_prefix(),
                        <Self as Name>::first_name(),
                        <Self as Address>::city_suffix())
            }
            1 => {
                format!("{} {}",
                        <Self as Name>::first_name(),
                        <Self as Address>::city_suffix())
            }
            _ => {
                format!("{} {}",
                        <Self as Name>::last_name(),
                        <Self as Address>::city_suffix())
            }
        }
    }

    #[inline]
    fn street_name() -> String {
        match gen_range(0, 2) {
            0 => {
                format!("{} {}",
                        <Self as Name>::first_name(),
                        <Self as Address>::street_suffix())
            }
            _ => {
                format!("{} {}",
                        <Self as Name>::last_name(),
                        <Self as Address>::street_suffix())
            }
        }
    }

    #[inline]
    fn building_number() -> String {
        let number_format =
            take_one(<Self as Fake>::address_building_number_format_data());
        numerify_sym(number_format)
    }

    #[inline]
    fn street_address() -> String {
        format!("{} {}",
                <Self as Address>::building_number(),
                <Self as Address>::street_name())
    }

    #[inline]
    fn secondary_address() -> String {
        format!("{} {}",
                take_one(&["Apt.", "Suit."]),
                <Self as Number>::number(3))
    }

    #[inline]
    fn zip() -> String {
        let number_format = take_one(<Self as Fake>::address_zip_format_data());
        numerify_sym(number_format)
    }

    #[inline]
    fn postcode() -> String {
        let number_format =
            take_one(<Self as Fake>::address_postcode_format_data());
        numerify_sym(number_format)
    }

    #[inline]
    fn latitude() -> String {
        ((random::<f64>() * 180_f64) - 90_f64).to_string()
    }

    #[inline]
    fn longitude() -> String {
        ((random::<f64>() * 360_f64) - 180_f64).to_string()
    }
}
