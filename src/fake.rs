use crate::locales::en;

pub trait Fake: Sized {
    #[inline(always)]
    fn lorem_word_data() -> &'static [&'static str] {
        en::data::LOREM_WORD
    }

    #[inline(always)]
    fn name_first_name_data() -> &'static [&'static str] {
        en::data::NAME_FIRST_NAME
    }

    #[inline(always)]
    fn name_last_name_data() -> &'static [&'static str] {
        en::data::NAME_LAST_NAME
    }

    #[inline(always)]
    fn name_prefix_data() -> &'static [&'static str] {
        en::data::NAME_PREFIX
    }

    #[inline(always)]
    fn name_suffix_data() -> &'static [&'static str] {
        en::data::NAME_SUFFIX
    }

    #[inline(always)]
    fn name_title_descriptor_data() -> &'static [&'static str] {
        en::data::NAME_TITLE_DESCRIPTOR
    }

    #[inline(always)]
    fn name_title_level_data() -> &'static [&'static str] {
        en::data::NAME_TITLE_LEVEL
    }

    #[inline(always)]
    fn name_title_job_data() -> &'static [&'static str] {
        en::data::NAME_TITLE_JOB
    }

    #[inline(always)]
    fn internet_free_email_provider_data() -> &'static [&'static str] {
        en::data::INTERNET_FREE_EMAIL_PROVIDER
    }

    #[inline(always)]
    fn internet_domain_suffix_data() -> &'static [&'static str] {
        en::data::INTERNET_DOMAIN_SUFFIX
    }

    #[inline(always)]
    fn internet_password_chars_data() -> &'static [char] {
        en::data::INTERNET_PASSWORD_CHARS
    }

    #[inline(always)]
    fn internet_user_agent_data() -> &'static [&'static str] {
        en::data::INTERNET_USER_AGENT
    }

    #[inline(always)]
    fn company_suffix_data() -> &'static [&'static str] {
        en::data::COMPANY_SUFFIX
    }

    #[inline(always)]
    fn company_buzzword_head_data() -> &'static [&'static str] {
        en::data::COMPANY_BUZZWORD_HEAD
    }

    #[inline(always)]
    fn company_buzzword_middle_data() -> &'static [&'static str] {
        en::data::COMPANY_BUZZWORD_MIDDLE
    }

    #[inline(always)]
    fn company_buzzword_tail_data() -> &'static [&'static str] {
        en::data::COMPANY_BUZZWORD_TAIL
    }

    #[inline(always)]
    fn company_bs_verbs_data() -> &'static [&'static str] {
        en::data::COMPANY_BS_VERBS
    }

    #[inline(always)]
    fn company_bs_adj_data() -> &'static [&'static str] {
        en::data::COMPANY_BS_ADJ
    }

    #[inline(always)]
    fn company_bs_nonus_data() -> &'static [&'static str] {
        en::data::COMPANY_BS_NONUS
    }

    #[inline(always)]
    fn company_industry_data() -> &'static [&'static str] {
        en::data::COMPANY_INDUSTRY
    }

    #[inline(always)]
    fn company_profession_data() -> &'static [&'static str] {
        en::data::COMPANY_PROFESSION
    }

    #[inline(always)]
    fn address_city_prefix_data() -> &'static [&'static str] {
        en::data::ADDRESS_CITY_PREFIX
    }

    #[inline(always)]
    fn address_city_suffix_data() -> &'static [&'static str] {
        en::data::ADDRESS_CITY_SUFFIX
    }

    #[inline(always)]
    fn address_country_data() -> &'static [&'static str] {
        en::data::ADDRESS_COUNTRY
    }

    #[inline(always)]
    fn address_country_code_data() -> &'static [&'static str] {
        en::data::ADDRESS_COUNTRY_CODE
    }

    #[inline(always)]
    fn address_street_suffix_data() -> &'static [&'static str] {
        en::data::ADDRESS_STREET_SUFFIX
    }

    #[inline(always)]
    fn address_time_zone_data() -> &'static [&'static str] {
        en::data::ADDRESS_TIME_ZONE
    }

    #[inline(always)]
    fn address_state_data() -> &'static [&'static str] {
        en::data::ADDRESS_STATE
    }

    #[inline(always)]
    fn address_state_abbr_data() -> &'static [&'static str] {
        en::data::ADDRESS_STATE_ABBR
    }

    #[inline(always)]
    fn address_building_number_format_data() -> &'static [&'static str] {
        en::data::ADDRESS_BUILDING_NUMBER_FORMAT
    }

    #[inline(always)]
    fn address_zip_format_data() -> &'static [&'static str] {
        en::data::ADDRESS_ZIP_FORMAT
    }

    #[inline(always)]
    fn address_postcode_format_data() -> &'static [&'static str] {
        en::data::ADDRESS_POSTCODE_FORMAT
    }

    #[inline(always)]
    fn phone_number_format_data() -> &'static [&'static str] {
        en::data::PHONE_NUMBER_FORMAT
    }

    #[inline(always)]
    fn phone_cell_number_format_data() -> &'static [&'static str] {
        en::data::PHONE_CELL_NUMBER_FORMAT
    }
}
