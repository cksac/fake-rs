extern crate rand;

pub mod helper;
pub mod locales;
pub use locales::en::Faker;
mod fake;
pub use fake::Fake;
pub mod faker;
pub mod dummy;
pub use dummy::Dummy;

#[macro_export]
macro_rules! fake {
    (@inner expr $e:expr) => ($e);
    (@inner $cat:ident . $m:ident ($($args:expr),+) in $locale:ident | $($func:ident)|+) => (
        Some(<$crate::locales::$locale::Faker as $crate::faker::$cat>::$m($($args),+))
        $(
        .map(|v| $func(v))
        )+
        .unwrap()
    );
    (@inner $cat:ident . $m:ident in $locale:ident | $($func:ident)|+) => (
        Some(<$crate::locales::$locale::Faker as $crate::faker::$cat>::$m())
        $(
        .map(|v| $func(v))
        )+
        .unwrap()
    );
    (@inner $cat:ident . $m:ident | $($func:ident)|+) => (
        Some(<$crate::locales::en::Faker as $crate::faker::$cat>::$m())
        $(
        .map(|v| $func(v))
        )+
        .unwrap()
    );
    (@inner $cat:ident . $m:ident ($($args:expr),+) | $($func:ident)|+) => (
        Some(<$crate::locales::en::Faker as $crate::faker::$cat>::$m($($args),+))
        $(
        .map(|v| $func(v))
        )+
        .unwrap()
    );
    (@inner $cat:ident . $m:ident ($($args:expr),+) in $locale:ident) => (<$crate::locales::$locale::Faker as $crate::faker::$cat>::$m($($args),+));
    (@inner $cat:ident . $m:ident in $locale:ident) => (<$crate::locales::$locale::Faker as $crate::faker::$cat>::$m());
    (@inner $cat:ident . $m:ident) => (<$crate::locales::en::Faker as $crate::faker::$cat>::$m());
    (@inner $cat:ident . $m:ident ($($args:expr),+)) => (<$crate::locales::en::Faker as $crate::faker::$cat>::$m($($args),+));

    ($fmt:expr, $([$named:ident = $($f:tt)+]),+) => (format!($fmt, $($named = fake!(@inner $($f)+)),+));
    ($fmt:expr, $([$($f:tt)+]),+) => (format!($fmt, $(fake!(@inner $($f)+)),+));
    ($cat:ident . $m:ident ($($args:expr),+) in $locale:ident) => (<$crate::locales::$locale::Faker as $crate::faker::$cat>::$m($($args),+));
    ($cat:ident . $m:ident in $locale:ident) => (<$crate::locales::$locale::Faker as $crate::faker::$cat>::$m());
    ($cat:ident . $m:ident ($($args:expr),+)) => (<$crate::locales::en::Faker as $crate::faker::$cat>::$m($($args),+));
    ($cat:ident . $m:ident) => (<$crate::locales::en::Faker as $crate::faker::$cat>::$m());
}

macro_rules! dummy {
    ($($d:tt)+) => (<$($d)+ as $crate::dummy::Dummy>::dummy());
}

#[cfg(test)]
mod tests {
    use super::Dummy;
    use super::faker::*;
    use super::helper::gen_range;

    fn to_lowercase<S: Into<String>>(s: S) -> String {
        s.into().to_lowercase()
    }

    #[test]
    fn test_custom_string() {
        let data = fake!("{} - {} - {}", [Name.name | to_lowercase], [Number.number(10)], [expr fake!(Name.name).to_lowercase()]);
        println!("{}", data);

        let name_locale = fake!("{} - {}", [Name.name], [Name.name in zh_tw]);
        println!("{}", name_locale);

        let response = fake!(r#"{{"name": "{x}", "chinese_name": "{y}"}}"#, [y = Name.name in zh_tw], [x = Name.name]);
        println!("{}", response);

        let image_url = fake!(r#"http://{domain}.{domain_suffix}/user/{username}.png?size={size}x{size}"#,
            [domain = Name.last_name | to_lowercase],
            [domain_suffix = Internet.domain_suffix],
            [username = Name.first_name | to_lowercase],
            [size = expr [512, 256, 128][gen_range(0, 3)]]
        );
        println!("{}", image_url);
    }

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
        println!("{}", <zh_tw::Faker as Name>::name_with_middle());
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
        println!("{:?}",
                 <Faker as PhoneNumber>::phone_number_with_format("N###-####"));
        println!("{:?}", <Faker as PhoneNumber>::cell_number());
    }

    #[test]
    fn macro_test() {
        println!("{:?}", fake!(Lorem.word));
        println!("{:?}", fake!(Number.number(10)));
        println!("{:?}", fake!(Lorem.sentence(4, 6)));
        println!("{}", fake!(Name.name in zh_tw));

        println!("{:?}", dummy!(i32));
        println!("{:?}", dummy!(Vec<Vec<i32>>));
    }

    #[test]
    fn dummy_test() {
        println!("{:?}", i8::dummy());
        println!("{:?}", i16::dummy());
        println!("{:?}", i32::dummy());
        println!("{:?}", i64::dummy());
        println!("{:?}", u8::dummy());
        println!("{:?}", u16::dummy());
        println!("{:?}", u32::dummy());
        println!("{:?}", u64::dummy());
        println!("{:?}", isize::dummy());
        println!("{:?}", usize::dummy());
        println!("{:?}", f32::dummy());
        println!("{:?}", f64::dummy());

        println!("{:?}", bool::dummy());
        println!("{:?}", String::dummy());
        println!("{:?}", <&str as Dummy>::dummy());

        println!("{:?}", Vec::<i32>::dummy());
        println!("{:?}", Option::<i32>::dummy());

        println!("{:?}", Vec::<Vec<i32>>::dummy());
        println!("{:?}", Vec::<Option<i32>>::dummy());

        println!("{:?}", <Vec<Vec<i32>> as Dummy>::dummy());
    }
}
