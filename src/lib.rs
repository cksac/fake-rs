#[cfg(feature = "chrono")]
extern crate chrono;

#[cfg(feature = "http")]
extern crate http;

extern crate rand;

pub mod helper;
pub mod locales;
pub use crate::locales::en::Faker;
mod fake;
pub use crate::fake::Fake;
#[macro_use]
pub mod dummy;
pub mod faker;
pub use dummy::{any, Dummy, DummyAny};

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
    (@inner $cat:ident . $m:ident ($($args:expr),+) in $locale:ident) => (
        <$crate::locales::$locale::Faker as $crate::faker::$cat>::$m($($args),+)
    );
    (@inner $cat:ident . $m:ident in $locale:ident) => (
        <$crate::locales::$locale::Faker as $crate::faker::$cat>::$m()
    );
    (@inner $cat:ident . $m:ident) => (
        <$crate::locales::en::Faker as $crate::faker::$cat>::$m()
    );
    (@inner $cat:ident . $m:ident ($($args:expr),+)) => (
        <$crate::locales::en::Faker as $crate::faker::$cat>::$m($($args),+)
    );
    ($fmt:expr, $([$named:ident = $($f:tt)+]),+) => (
        format!($fmt, $($named = fake!(@inner $($f)+)),+)
    );
    ($fmt:expr, $([$($f:tt)+]),+) => (
        format!($fmt, $(fake!(@inner $($f)+)),+)
    );
    ($cat:ident . $m:ident ($($args:expr),+) in $locale:ident) => (
        <$crate::locales::$locale::Faker as $crate::faker::$cat>::$m($($args),+)
    );
    ($cat:ident . $m:ident in $locale:ident) => (
        <$crate::locales::$locale::Faker as $crate::faker::$cat>::$m()
    );
    ($cat:ident . $m:ident ($($args:expr),+)) => (
        <$crate::locales::en::Faker as $crate::faker::$cat>::$m($($args),+)
    );
    ($cat:ident . $m:ident) => (
        <$crate::locales::en::Faker as $crate::faker::$cat>::$m()
    );
}

#[cfg(test)]
mod tests {
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
        println!("{:?}", <Faker as Internet>::ip());
        println!("{:?}", <Faker as Internet>::ipv4());
        println!("{:?}", <Faker as Internet>::ipv6());
        println!("{:?}", <Faker as Internet>::color());
        println!("{:?}", <Faker as Internet>::user_agent());
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
        println!(
            "{:?}",
            <Faker as PhoneNumber>::phone_number_with_format("N###-####")
        );
        println!("{:?}", <Faker as PhoneNumber>::cell_number());
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn time_usage() {
        use chrono::prelude::*;

        println!("{:?}", <Faker as Chrono>::time(None));
        println!("{:?}", <Faker as Chrono>::time(Some("%I.%M.%S %p")));
        println!("{:?}", <Faker as Chrono>::date(None));
        println!("{:?}", <Faker as Chrono>::date(Some("%A %Y.%m.%d")));
        println!("{:?}", <Faker as Chrono>::duration());
        println!("{:?}", <Faker as Chrono>::datetime(None));
        println!("{:?}", <Faker as Chrono>::datetime(Some("%c")));
        println!("{:?}", <Faker as Chrono>::datetime(Some("%s")));

        let now = Utc::now();

        // test after
        let fmt = "%c";
        let now_str = now.format(fmt).to_string();
        println!("after: {:?}", <Faker as Chrono>::after(Some(fmt), &now_str));
        println!("after: {:?}", <Faker as Chrono>::after(Some(fmt), &now_str));

        // test before
        let fmt = "%a %b %e %T %Y %:z";
        let now_str = now.format(fmt).to_string();
        println!(
            "before: {:?}",
            <Faker as Chrono>::before(Some(fmt), &now_str)
        );
        println!(
            "before: {:?}",
            <Faker as Chrono>::before(Some(fmt), &now_str)
        );

        // test between
        let early = Utc.ymd(2010, 4, 20).and_hms(11, 11, 11);
        let late = Utc.ymd(2020, 6, 5).and_hms(9, 32, 33);

        println!(
            "between: {:?}",
            <Faker as Chrono>::between(None, &early.to_rfc3339(), &late.to_rfc3339())
        );

        let fmt = "%+";
        let date = <Faker as Chrono>::between(
            Some(fmt),
            &early.format(fmt).to_string(),
            &late.format(fmt).to_string(),
        );

        println!("between '{}': {:?}", fmt, date);

        let fmt = "%c";
        let date = <Faker as Chrono>::between(
            Some(fmt),
            &early.format(fmt).to_string(),
            &late.format(fmt).to_string(),
        );

        println!("between '{}': {:?}", fmt, date);
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn time_macro_test() {
        use chrono::prelude::*;

        // Copy paste below to README.md for example
        let early = Utc.ymd(2010, 4, 20).and_hms(11, 11, 11);
        let late = Utc.ymd(2020, 6, 5).and_hms(9, 32, 33);
        let now = Utc::now();
        let fmt = "%a %b %e %T %Y %:z";
        let now_str = now.format(fmt).to_string();

        println!("{:?}", fake!(Chrono.time(Some("%I.%M.%S %p"))));
        println!("{:?}", fake!(Chrono.date(Some("%A %Y.%m.%d"))));
        println!("{:?}", fake!(Chrono.datetime(None)));
        println!("{:?}", fake!(Chrono.before(Some(fmt), &now_str)));
        println!("{:?}", fake!(Chrono.after(Some(fmt), &now_str)));
        println!(
            "{:?}",
            fake!(Chrono.between(
                Some(fmt),
                &early.format(fmt).to_string(),
                &late.format(fmt).to_string()
            ))
        );
        println!(
            "{:?}",
            fake!(Chrono.between(None, &early.to_rfc3339(), &late.to_rfc3339()))
        )
    }

    #[cfg(feature = "http")]
    #[test]
    fn http_test() {
        use http;

        println!("{:?}", fake!(Http.status_code));
        println!("{:?}", fake!(Http.status_code).canonical_reason());

        println!("{:?}", fake!(Http.all_status_code));
        println!("{:?}", fake!(Http.all_status_code).canonical_reason());
    }

    #[test]
    fn macro_test() {
        println!("{:?}", fake!(Lorem.word));
        println!("{:?}", fake!(Number.number(10)));
        println!("{:?}", fake!(Lorem.sentence(4, 6)));
        println!("{}", fake!(Name.name in zh_tw));
    }
}
