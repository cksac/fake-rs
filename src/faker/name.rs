use ::helper::*;
use ::Fake;

pub trait Name: Fake {
    #[inline]
    fn first_name() -> &'static str {
        take_one(<Self as Fake>::name_first_name_data())
    }

    #[inline]
    fn last_name() -> &'static str {
        take_one(<Self as Fake>::name_last_name_data())
    }

    #[inline]
    fn prefix() -> &'static str {
        take_one(<Self as Fake>::name_prefix_data())
    }

    #[inline]
    fn suffix() -> &'static str {
        take_one(<Self as Fake>::name_suffix_data())
    }

    #[inline]
    fn name() -> String {
        match gen_range(0, 6) {
            0 => {
                format!("{} {} {}",
                        <Self as Name>::prefix(),
                        <Self as Name>::first_name(),
                        <Self as Name>::last_name())
            }
            1 => {
                format!("{} {} {}",
                        <Self as Name>::first_name(),
                        <Self as Name>::last_name(),
                        <Self as Name>::suffix())
            }
            _ => {
                format!("{} {}",
                        <Self as Name>::first_name(),
                        <Self as Name>::last_name())
            }
        }
    }

    #[inline]
    fn name_with_middle() -> String {
        match gen_range(0, 6) {
            0 => {
                format!("{} {} {} {}",
                        <Self as Name>::prefix(),
                        <Self as Name>::first_name(),
                        <Self as Name>::first_name(),
                        <Self as Name>::last_name())
            }
            1 => {
                format!("{} {} {} {}",
                        <Self as Name>::first_name(),
                        <Self as Name>::first_name(),
                        <Self as Name>::last_name(),
                        <Self as Name>::suffix())
            }
            _ => {
                format!("{} {} {}",
                        <Self as Name>::first_name(),
                        <Self as Name>::first_name(),
                        <Self as Name>::last_name())
            }
        }
    }

    #[inline]
    fn title_descriptor() -> &'static str {
        take_one(<Self as Fake>::name_title_descriptor_data())
    }

    #[inline]
    fn title_level() -> &'static str {
        take_one(<Self as Fake>::name_title_level_data())
    }

    #[inline]
    fn title_job() -> &'static str {
        take_one(<Self as Fake>::name_title_job_data())
    }

    #[inline]
    fn title() -> String {
        format!("{} {} {}",
                <Self as Name>::title_descriptor(),
                <Self as Name>::title_level(),
                <Self as Name>::title_job())
    }
}
