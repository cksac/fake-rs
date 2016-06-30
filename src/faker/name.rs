use ::helper::*;
use ::Fake;

pub trait Name {
    fn first_name() -> &'static str;
    fn last_name() -> &'static str;
    fn prefix() -> &'static str;
    fn suffix() -> &'static str;
    fn name() -> String;
    fn name_with_middle() -> String;
    fn title_descriptor() -> &'static str;
    fn title_level() -> &'static str;
    fn title_job() -> &'static str;
    fn title() -> String;
}

impl<T: Fake> Name for T {
    #[inline]
    default fn first_name() -> &'static str {
        T::NAME_FIRST_NAME[gen_range(0, T::NAME_FIRST_NAME.len())]
    }

    #[inline]
    default fn last_name() -> &'static str {
        T::NAME_LAST_NAME[gen_range(0, T::NAME_LAST_NAME.len())]
    }

    #[inline]
    default fn prefix() -> &'static str {
        T::NAME_PREFIX[gen_range(0, T::NAME_PREFIX.len())]
    }

    #[inline]
    default fn suffix() -> &'static str {
        T::NAME_SUFFIX[gen_range(0, T::NAME_SUFFIX.len())]
    }

    #[inline]
    default fn name() -> String {
        match gen_range(0, 6) {
            0 => format!("{} {} {}", <Self as Name>::prefix(), <Self as Name>::first_name(), <Self as Name>::last_name()),
            1 => format!("{} {} {}", <Self as Name>::first_name(), <Self as Name>::last_name(), <Self as Name>::suffix()),
            _ => format!("{} {}", <Self as Name>::first_name(), <Self as Name>::last_name()),
        }
    }

    #[inline]
    default fn name_with_middle() -> String {
        match gen_range(0, 6) {
            0 => format!("{} {} {} {}", <Self as Name>::prefix(), <Self as Name>::first_name(), <Self as Name>::first_name(), <Self as Name>::last_name()),
            1 => format!("{} {} {} {}", <Self as Name>::first_name(), <Self as Name>::first_name(), <Self as Name>::last_name(), <Self as Name>::suffix()),
            _ => format!("{} {} {}", <Self as Name>::first_name(), <Self as Name>::first_name(), <Self as Name>::last_name()),
        }
    }

    #[inline]
    default fn title_descriptor() -> &'static str {
        T::NAME_TITLE_DESCRIPTOR[gen_range(0, T::NAME_TITLE_DESCRIPTOR.len())]
    }

    #[inline]
    default fn title_level() -> &'static str {
        T::NAME_TITLE_LEVEL[gen_range(0, T::NAME_TITLE_LEVEL.len())]
    }

    #[inline]
    default fn title_job() -> &'static str {
        T::NAME_TITLE_JOB[gen_range(0, T::NAME_TITLE_JOB.len())]
    }

    #[inline]
    default fn title() -> String {
        format!("{} {} {}", <Self as Name>::title_descriptor(), <Self as Name>::title_level(), <Self as Name>::title_job())
    }
}
