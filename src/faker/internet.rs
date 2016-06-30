use ::helper::*;
use ::Fake;
use ::faker::{Name, Number, Lorem};
use ::locales::en;

pub trait Internet {
    fn free_email_provider() -> &'static str;
    fn domain_suffix() -> &'static str;
    fn user_name() -> String;
    fn free_email() -> String;
    fn safe_email() -> String;
}

impl<T: Fake> Internet for T {
    #[inline]
    default fn free_email_provider() -> &'static str {
        T::INTERNET_FREE_EMAIL_PROVIDER[gen_range(0, T::INTERNET_FREE_EMAIL_PROVIDER.len())]
    }

    #[inline]
    default fn domain_suffix() -> &'static str {
        T::INTERNET_DOMAIN_SUFFIX[gen_range(0, T::INTERNET_DOMAIN_SUFFIX.len())]
    }

    #[inline]
    default fn user_name() -> String {
        match gen_range(0, 10) {
            0 => <en::Faker as Name>::first_name().replace("'", "").to_lowercase(),
            1 | 2 => {
                format!("{}.{}",
                        <en::Faker as Lorem>::word().to_lowercase(),
                        <en::Faker as Name>::first_name().replace("'", "").to_lowercase())
            }
            3 | 4 => {
                format!("{}{}",
                        <en::Faker as Name>::first_name().replace("'", "").to_lowercase(),
                        <en::Faker as Number>::number(4))
            }
            _ => {
                format!("{}_{}",
                        <en::Faker as Name>::first_name().replace("'", "").to_lowercase(),
                        <en::Faker as Lorem>::word().to_lowercase())
            }
        }
    }

    #[inline]
    default fn free_email() -> String {
        format!("{}@{}",
                <T as Internet>::user_name(),
                <T as Internet>::free_email_provider())
    }

    #[inline]
    default fn safe_email() -> String {
        format!("{}@example.{}",
                <en::Faker as Name>::first_name().to_lowercase(),
                ["com", "net", "org"][gen_range(0, 2)])
    }
}
