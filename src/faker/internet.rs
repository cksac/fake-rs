use Fake;
use faker::{Lorem, Name, Number};
use helper::*;
use locales::en;

pub trait Internet: Fake {
    #[inline]
    fn free_email_provider() -> &'static str {
        take_one(Self::internet_free_email_provider_data())
    }

    #[inline]
    fn domain_suffix() -> &'static str {
        take_one(Self::internet_domain_suffix_data())
    }

    #[inline]
    fn user_name() -> String {
        match gen_range(0, 10) {
            0 => <en::Faker as Name>::first_name()
                .replace("'", "")
                .to_lowercase(),
            1 | 2 => format!(
                "{}.{}",
                <en::Faker as Lorem>::word().to_lowercase(),
                <en::Faker as Name>::first_name()
                    .replace("'", "")
                    .to_lowercase()
            ),
            3 | 4 => format!(
                "{}{}",
                <en::Faker as Name>::first_name()
                    .replace("'", "")
                    .to_lowercase(),
                <en::Faker as Number>::number(4)
            ),
            _ => format!(
                "{}_{}",
                <en::Faker as Name>::first_name()
                    .replace("'", "")
                    .to_lowercase(),
                <en::Faker as Lorem>::word().to_lowercase()
            ),
        }
    }

    #[inline]
    fn free_email() -> String {
        format!("{}@{}", Self::user_name(), Self::free_email_provider())
    }

    #[inline]
    fn safe_email() -> String {
        format!(
            "{}@example.{}",
            <en::Faker as Name>::first_name()
                .replace("'", "")
                .to_lowercase(),
            take_one(&["com", "net", "org"])
        )
    }

    #[inline]
    fn password(min_count: usize, max_count: usize) -> String {
        let length = gen_range(min_count, max_count + 1);
        let mut v = shuffle(Self::internet_password_chars_data());
        while v.len() < length {
            v.append(&mut shuffle(Self::internet_password_chars_data()));
        }
        v.truncate(length);
        v.into_iter().collect()
    }

    #[inline]
    fn ip() -> String {
        Self::ipv4()
    }

    #[inline]
    fn ipv4() -> String {
        format!(
            "{}.{}.{}.{}",
            gen_range(1, 256),
            gen_range(1, 256),
            gen_range(1, 256),
            gen_range(1, 256)
        )
    }

    #[inline]
    fn ipv6() -> String {
        format!(
            "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}",
            gen_range(1, 65536),
            gen_range(1, 65536),
            gen_range(1, 65536),
            gen_range(1, 65536),
            gen_range(1, 65536),
            gen_range(1, 65536),
            gen_range(1, 65536),
            gen_range(1, 65536)
        )
    }

    #[inline]
    fn color() -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            gen_range(1, 256),
            gen_range(1, 256),
            gen_range(1, 256)
        )
    }
}
