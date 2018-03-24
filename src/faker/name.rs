use Fake;
use helper::*;

pub trait Name: Fake {
    #[inline]
    fn first_name() -> &'static str {
        take_one(Self::name_first_name_data())
    }

    #[inline]
    fn last_name() -> &'static str {
        take_one(Self::name_last_name_data())
    }

    #[inline]
    fn prefix() -> &'static str {
        take_one(Self::name_prefix_data())
    }

    #[inline]
    fn suffix() -> &'static str {
        take_one(Self::name_suffix_data())
    }

    #[inline]
    fn name() -> String {
        match gen_range(0, 6) {
            0 => format!(
                "{} {} {}",
                Self::prefix(),
                Self::first_name(),
                Self::last_name()
            ),
            1 => format!(
                "{} {} {}",
                Self::first_name(),
                Self::last_name(),
                Self::suffix()
            ),
            _ => format!("{} {}", Self::first_name(), Self::last_name()),
        }
    }

    #[inline]
    fn name_with_middle() -> String {
        match gen_range(0, 6) {
            0 => format!(
                "{} {} {} {}",
                Self::prefix(),
                Self::first_name(),
                Self::first_name(),
                Self::last_name()
            ),
            1 => format!(
                "{} {} {} {}",
                Self::first_name(),
                Self::first_name(),
                Self::last_name(),
                Self::suffix()
            ),
            _ => format!(
                "{} {} {}",
                Self::first_name(),
                Self::first_name(),
                Self::last_name()
            ),
        }
    }

    #[inline]
    fn title_descriptor() -> &'static str {
        take_one(Self::name_title_descriptor_data())
    }

    #[inline]
    fn title_level() -> &'static str {
        take_one(Self::name_title_level_data())
    }

    #[inline]
    fn title_job() -> &'static str {
        take_one(Self::name_title_job_data())
    }

    #[inline]
    fn title() -> String {
        format!(
            "{} {} {}",
            Self::title_descriptor(),
            Self::title_level(),
            Self::title_job()
        )
    }
}
