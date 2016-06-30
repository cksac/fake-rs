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
mod lorem;
pub use lorem::Lorem;
mod name;
pub use name::Name;
mod number;
pub use number::Number;
mod boolean;
pub use boolean::Boolean;


#[cfg(test)]
mod tests {
    use super::{Fake, Lorem, Name, Number, Boolean};
    use super::Faker;

    #[test]
    fn lorem_usage() {
        println!("{:?}", <Faker as Lorem>::word());
        println!("{:?}", <Faker as Lorem>::words(10));
        println!("{:?}", <Faker as Lorem>::sentence(4, 6));
        println!("{:?}", <Faker as Lorem>::sentences(10));
        println!("{:?}", <Faker as Lorem>::paragraph(7, 3));
        println!("{:?}", <Faker as Lorem>::paragraphs(3));

        println!("{:?}", Faker::lorem_word());
        println!("{:?}", Faker::lorem_words(10));
        println!("{:?}", Faker::lorem_sentence(4, 6));
        println!("{:?}", Faker::lorem_sentences(10));
        println!("{:?}", Faker::lorem_paragraph(7, 3));
        println!("{:?}", Faker::lorem_paragraphs(3));
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
}
