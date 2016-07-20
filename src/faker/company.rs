use ::helper::*;
use ::Fake;
use ::faker::Name;

pub trait Company: Fake + Name {
    #[inline]
    fn suffix() -> &'static str {
        take_one(<Self as Fake>::company_suffix_data())
    }

    #[inline]
    fn name() -> String {
        match gen_range(0, 8) {
            0 => {
                format!("{}-{}",
                        <Self as Name>::last_name(),
                        <Self as Name>::last_name())
            }
            1 | 2 => {
                format!("{}, {} and {}",
                        <Self as Name>::last_name(),
                        <Self as Name>::last_name(),
                        <Self as Name>::last_name())
            }
            _ => {
                format!("{} {}",
                        <Self as Name>::last_name(),
                        <Self as Company>::suffix())
            }
        }
    }

    #[inline]
    fn buzzword() -> &'static str {
        take_one(<Self as Fake>::company_buzzword_head_data())
    }

    #[inline]
    fn catch_phase() -> String {
        let head = take_one(<Self as Fake>::company_buzzword_head_data());
        let middle = take_one(<Self as Fake>::company_buzzword_middle_data());
        let tail = take_one(<Self as Fake>::company_buzzword_tail_data());
        format!("{} {} {}", head, middle, tail)
    }

    #[inline]
    fn bs() -> String {
        let head = take_one(<Self as Fake>::company_bs_verbs_data());
        let middle = take_one(<Self as Fake>::company_bs_adj_data());
        let tail = take_one(<Self as Fake>::company_bs_nonus_data());
        format!("{} {} {}", head, middle, tail)
    }

    #[inline]
    fn profession() -> &'static str {
        take_one(<Self as Fake>::company_profession_data())
    }

    #[inline]
    fn industry() -> &'static str {
        take_one(<Self as Fake>::company_industry_data())
    }
}
