#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![feature(associated_consts)]
#![feature(specialization)]

extern crate rand;

pub mod helper;
pub mod locales;
pub use locales::en::Faker;
mod fake;
pub use fake::Fake;
pub mod faker;


#[cfg(test)]
mod tests {
    use super::faker::*;

    #[test]
    fn lorem_usage() {
        println!("{:?}", <Faker as Lorem>::word());
        println!("{:?}", <Faker as Lorem>::words(10));
        println!("{:?}", <Faker as Lorem>::sentence(4, 6));
        println!("{:?}", <Faker as Lorem>::sentences(10));
        println!("{:?}", <Faker as Lorem>::paragraph(7, 3));
        println!("{:?}", <Faker as Lorem>::paragraphs(3));
    }

    #[test]
    fn name_usage() {
        println!("{:?}", <Faker as Name>::first_name());
        println!("{:?}", <Faker as Name>::last_name());
        println!("{:?}", <Faker as Name>::name());
        println!("{:?}", <Faker as Name>::name_with_middle());
        println!("{:?}", <Faker as Name>::title_descriptor());
        println!("{:?}", <Faker as Name>::title_level());
        println!("{:?}", <Faker as Name>::title_job());
        println!("{:?}", <Faker as Name>::title());

        use super::locales::zh_tw;
        println!("{}", <zh_tw::Faker as Name>::first_name());
        println!("{}", <zh_tw::Faker as Name>::last_name());
        println!("{}", <zh_tw::Faker as Name>::name());
        println!("{:?}", <zh_tw::Faker as Name>::name_with_middle());
        println!("{:?}", <zh_tw::Faker as Name>::title_descriptor());
        println!("{:?}", <zh_tw::Faker as Name>::title_level());
        println!("{:?}", <zh_tw::Faker as Name>::title_job());
        println!("{:?}", <zh_tw::Faker as Name>::title());
    }

    #[test]
    fn number_usage() {
        println!("{:?}", <Faker as Number>::digit());
        println!("{:?}", <Faker as Number>::number(10));
        println!("{:?}", <Faker as Number>::between(5, 10));
        println!("{:?}", <Faker as Number>::between(5.0_f32, 10.0_f32));
    }

    #[test]
    fn boolean_usage() {
        println!("{:?}", <Faker as Boolean>::boolean());
    }

    #[test]
    fn internet_usage() {
        println!("{:?}", <Faker as Internet>::free_email_provider());
        println!("{:?}", <Faker as Internet>::domain_suffix());
        println!("{:?}", <Faker as Internet>::user_name());
        println!("{:?}", <Faker as Internet>::free_email());
        println!("{:?}", <Faker as Internet>::safe_email());
        println!("{:?}", <Faker as Internet>::password(8, 20));
    }

    #[test]
    fn company_usage() {
        println!("{:?}", <Faker as Company>::suffix());
        println!("{:?}", <Faker as Company>::name());
        println!("{:?}", <Faker as Company>::buzzword());
        println!("{:?}", <Faker as Company>::catch_phase());
        println!("{:?}", <Faker as Company>::bs());
        println!("{:?}", <Faker as Company>::profession());
        println!("{:?}", <Faker as Company>::industry());
    }

    #[test]
    fn address_usage() {
        println!("{:?}", <Faker as Address>::time_zone());
        println!("{:?}", <Faker as Address>::city_prefix());
        println!("{:?}", <Faker as Address>::city_suffix());
        println!("{:?}", <Faker as Address>::street_suffix());
        println!("{:?}", <Faker as Address>::state());
        println!("{:?}", <Faker as Address>::state_abbr());
        println!("{:?}", <Faker as Address>::city());
        println!("{:?}", <Faker as Address>::street_name());
        println!("{:?}", <Faker as Address>::building_number());
        println!("{:?}", <Faker as Address>::street_address());
        println!("{:?}", <Faker as Address>::secondary_address());
        println!("{:?}", <Faker as Address>::zip());
        println!("{:?}", <Faker as Address>::postcode());
        println!("{:?}", <Faker as Address>::latitude());
        println!("{:?}", <Faker as Address>::longitude());
    }

    #[test]
    fn phone_number_usage() {
        println!("{:?}", <Faker as PhoneNumber>::phone_number());
        println!("{:?}", <Faker as PhoneNumber>::phone_number_with_format("N#######"));
        println!("{:?}", <Faker as PhoneNumber>::cell_number());
    }
}
