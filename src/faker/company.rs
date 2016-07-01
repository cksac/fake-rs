use ::helper::*;
use ::Fake;
use ::faker::Name;

pub trait Company {
    fn suffix() -> &'static str;
    fn name() -> String;
    fn buzzword() -> String;
    fn catch_phase() -> String;
    fn bs() -> String;
    fn profession() -> &'static str;
    fn industry() -> &'static str;
}

impl<T: Fake> Company for T {
    #[inline]
    default fn suffix() -> &'static str {
        T::COMPANY_SUFFIX[gen_range(0, T::COMPANY_SUFFIX.len())]
    }

    #[inline]
    default fn name() -> String {
        match gen_range(0, 8) {
            0 => format!("{}-{}", <T as Name>::last_name(), <T as Name>::last_name()),
            1 | 2 => {
                format!("{}, {} and {}",
                        <T as Name>::last_name(),
                        <T as Name>::last_name(),
                        <T as Name>::last_name())
            }
            _ => format!("{} {}", <T as Name>::last_name(), <T as Company>::suffix()),
        }
    }

    #[inline]
    default fn buzzword() -> String {
        T::COMPANY_BUZZWORD_HEAD[gen_range(0, T::COMPANY_BUZZWORD_HEAD.len())].to_string()
    }

    #[inline]
    default fn catch_phase() -> String {
        let head = T::COMPANY_BUZZWORD_HEAD[gen_range(0, T::COMPANY_BUZZWORD_HEAD.len())];
        let middle = T::COMPANY_BUZZWORD_MIDDLE[gen_range(0, T::COMPANY_BUZZWORD_MIDDLE.len())];
        let tail = T::COMPANY_BUZZWORD_TAIL[gen_range(0, T::COMPANY_BUZZWORD_TAIL.len())];
        format!("{} {} {}", head, middle, tail)
    }

    #[inline]
    default fn bs() -> String {
        let head = T::COMPANY_BS_VERBS[gen_range(0, T::COMPANY_BS_VERBS.len())];
        let middle = T::COMPANY_BS_ADJ[gen_range(0, T::COMPANY_BS_ADJ.len())];
        let tail = T::COMPANY_BS_NONUS[gen_range(0, T::COMPANY_BS_NONUS.len())];
        format!("{} {} {}", head, middle, tail)
    }

    #[inline]
    default fn profession() -> &'static str {
        T::COMPANY_PROFESSION[gen_range(0, T::COMPANY_PROFESSION.len())]
    }

    #[inline]
    default fn industry() -> &'static str {
        T::COMPANY_INDUSTRY[gen_range(0, T::COMPANY_INDUSTRY.len())]
    }
}
