use fake::{Fake, Faker, locales::* };
use rand::SeedableRng as _;

macro_rules! check_determinism {
    (one $name:ident, $ty:ty, $op:expr) => {
        proptest::proptest! {
            #[allow(unused_parens)]
            #[test]
            fn $name(seed: [u8; 32]) {
                let mut rng1 = rand_chacha::ChaCha20Rng::from_seed(seed);
                let mut rng2 = rng1.clone();

                for _ in 0..16 {
                    let val1: $ty = $op.fake_with_rng(&mut rng1);
                    let val2: $ty = $op.fake_with_rng(&mut rng2);

                    proptest::prop_assert_eq!(val1, val2);
                }
            }
        }
    };
    ($op:expr; $name:ident, $ty:ty, $($tt:tt),*) => {
        check_determinism!(one $name, $ty, $op);
        check_determinism!($op; $($tt),*);
    };
    ($op:expr; $name:ident, $ty:ty) => {
        check_determinism!(one $name, $ty, $op);
    };
    (l10d $op:expr; $ty:ty, $name_en:ident, $name_fr:ident, $name_cn:ident, $name_tw:ident $(, $attrs:expr)*) => {
        check_determinism!(one $name_en, $ty, $op(EN $(, $attrs)*));
        check_determinism!(one $name_fr, $ty, $op(FR_FR $(, $attrs)*));
        check_determinism!(one $name_cn, $ty, $op(ZH_CN $(, $attrs)*));
        check_determinism!(one $name_tw, $ty, $op(ZH_TW $(, $attrs)*));
    };
}

check_determinism! {
    Faker; check_bool, bool, check_u8, u8, check_u16, u16, check_u32, u32, check_u64, u64, check_u128, u128,
    check_usize, usize, check_i8, i8, check_i16, i16, check_i32, i32, check_i64, i64, check_i128, i128,
    check_isize, isize
}

check_determinism! { one fake_u64_range, u64, (1u64..54683546546434) }

// Address
use fake::faker::address::raw::*;

check_determinism! { l10d BuildingNumber; String, fake_buildingnumber_en, fake_buildingnumber_fr, fake_buildingnumber_cn, fake_buildingnumber_tw }
check_determinism! { l10d CityName; String, fake_cityname_en, fake_cityname_fr, fake_cityname_cn, fake_cityname_tw }
check_determinism! { l10d CityPrefix; String, fake_cityprefix_en, fake_cityprefix_fr, fake_cityprefix_cn, fake_cityprefix_tw }
check_determinism! { l10d CitySuffix; String, fake_citysuffix_en, fake_citysuffix_fr, fake_citysuffix_cn, fake_citysuffix_tw }
check_determinism! { l10d CountryCode; String, fake_countrycode_en, fake_countrycode_fr, fake_countrycode_cn, fake_countrycode_tw }
check_determinism! { l10d CountryName; String, fake_countryname_en, fake_countryname_fr, fake_countryname_cn, fake_countryname_tw }
check_determinism! { l10d Geohash; String, fake_geohash_en, fake_geohash_fr, fake_geohash_cn, fake_geohash_tw, 11 }
check_determinism! { l10d Latitude; String, fake_latitude_en, fake_latitude_fr, fake_latitude_cn, fake_latitude_tw }
check_determinism! { l10d Longitude; String, fake_longitude_en, fake_longitude_fr, fake_longitude_cn, fake_longitude_tw }
check_determinism! { l10d PostCode; String, fake_postcode_en, fake_postcode_fr, fake_postcode_cn, fake_postcode_tw }
check_determinism! { l10d SecondaryAddress; String, fake_secondary_address_en, fake_secondary_address_fr, fake_secondary_address_cn, fake_secondary_address_tw }
check_determinism! { l10d SecondaryAddressType; String, fake_secondary_address_type_en, fake_secondary_address_type_fr, fake_secondary_address_type_cn, fake_secondary_address_type_tw }
check_determinism! { l10d StateAbbr; String, fake_state_abbr_en, fake_state_abbr_fr, fake_state_abbr_cn, fake_state_abbr_tw }
check_determinism! { l10d StateName; String, fake_state_name_en, fake_state_name_fr, fake_state_name_cn, fake_state_name_tw }
check_determinism! { l10d StreetName; String, fake_street_name_en, fake_street_name_fr, fake_street_name_cn, fake_street_name_tw }
check_determinism! { l10d StreetSuffix; String, fake_street_suffix_en, fake_street_suffix_fr, fake_street_suffix_cn, fake_street_suffix_tw }
check_determinism! { l10d TimeZone; String, fake_time_zone_en, fake_time_zone_fr, fake_time_zone_cn, fake_time_zone_tw }
check_determinism! { l10d ZipCode; String, fake_zip_code_en, fake_zip_code_fr, fake_zip_code_cn, fake_zip_code_tw }

// Administrative
use fake::faker::administrative::raw::*;

check_determinism! { one fake_health_insurance_code_fr, String, HealthInsuranceCode(FR_FR) }

// Automotive
use fake::faker::automotive::raw::*;

check_determinism! { one fake_license_plate_fr, String, LicencePlate(FR_FR) }

// Barcode
use fake::faker::barecode::raw::*;

check_determinism! { l10d Isbn; String, fake_isbn_en, fake_isbn_fr, fake_isbn_cn, fake_isbn_tw }
check_determinism! { l10d Isbn10; String, fake_isbn10_en, fake_isbn10_fr, fake_isbn10_cn, fake_isbn10_tw }
check_determinism! { l10d Isbn13; String, fake_isbn13_en, fake_isbn13_fr, fake_isbn13_cn, fake_isbn13_tw }

// Chrono
#[cfg(feature = "chrono")]
mod chrono {
    use fake::{faker::chrono::raw::*, Fake, locales::* };
    use rand::SeedableRng as _;

    fn lo() -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_utc(chrono::NaiveDateTime::from_timestamp(53469346924, 124241), chrono::Utc)
    }

    fn hi() -> chrono::DateTime<chrono::Utc> {
        lo() + chrono::Duration::days(365)
    }

    check_determinism! { l10d Date; String, fake_date_en, fake_date_fr, fake_date_cn, fake_date_tw }
    check_determinism! { l10d DateTime; String, fake_date_time_en, fake_date_time_fr, fake_date_time_cn, fake_date_time_tw }
    check_determinism! { l10d DateTimeAfter; String, fake_date_time_after_en, fake_date_time_after_fr, fake_date_time_after_cn, fake_date_time_after_tw, lo() }
    check_determinism! { l10d DateTimeBefore; String, fake_date_time_before_en, fake_date_time_before_fr, fake_date_time_before_cn, fake_date_time_before_tw, hi() }
    check_determinism! { l10d DateTimeBetween; String, fake_date_time_between_en, fake_date_time_between_fr, fake_date_time_between_cn, fake_date_time_between_tw, lo(), hi() }
    check_determinism! { l10d Duration; ::chrono::Duration, fake_duration_en, fake_duration_fr, fake_duration_cn, fake_duration_tw }
    check_determinism! { l10d Time; String, fake_time_en, fake_time_fr, fake_time_cn, fake_time_tw }
}

// Company
use fake::faker::company::raw::*;

check_determinism! { l10d Bs; String, fake_bs_en, fake_bs_fr, fake_bs_cn, fake_bs_tw }
check_determinism! { l10d BsAdj; String, fake_bs_adj_en, fake_bs_adj_fr, fake_bs_adj_cn, fake_bs_adj_tw }
check_determinism! { l10d BsNoun; String, fake_bs_noun_en, fake_bs_noun_fr, fake_bs_noun_cn, fake_bs_noun_tw }
check_determinism! { l10d BsVerb; String, fake_bs_verb_en, fake_bs_verb_fr, fake_bs_verb_cn, fake_bs_verb_tw }
check_determinism! { l10d Buzzword; String, fake_buzzword_en, fake_buzzword_fr, fake_buzzword_cn, fake_buzzword_tw }
check_determinism! { l10d BuzzwordMiddle; String, fake_buzzword_middle_en, fake_buzzword_middle_fr, fake_buzzword_middle_cn, fake_buzzword_middle_tw }
check_determinism! { l10d BuzzwordTail; String, fake_buzzword_tail_en, fake_buzzword_tail_fr, fake_buzzword_tail_cn, fake_buzzword_tail_tw }
check_determinism! { l10d CatchPhase; String, fake_catchphrase_en, fake_catchphrase_fr, fake_catchphrase_cn, fake_catchphrase_tw }
check_determinism! { l10d CompanyName; String, fake_company_name_en, fake_company_name_fr, fake_company_name_cn, fake_company_name_tw }
check_determinism! { l10d CompanySuffix; String, fake_company_suffix_en, fake_company_suffix_fr, fake_company_suffix_cn, fake_company_suffix_tw }
check_determinism! { l10d Industry; String, fake_industry_en, fake_industry_fr, fake_industry_cn, fake_industry_tw }
check_determinism! { l10d Profession; String, fake_profession_en, fake_profession_fr, fake_profession_cn, fake_profession_tw }

// Credit Card
use fake::faker::creditcard::raw::*;

// it's sufficient to check one language, because it doesn't change anything
check_determinism! { one fake_credit_card_number_en, String, CreditCardNumber(EN) }

// Currency
use fake::faker::currency::raw::*;

check_determinism! { l10d CurrencyCode; String, fake_currency_code_en, fake_currency_code_fr, fake_currency_code_cn, fake_currency_code_tw }
check_determinism! { l10d CurrencyName; String, fake_currency_name_en, fake_currency_name_fr, fake_currency_name_cn, fake_currency_name_tw }
check_determinism! { l10d CurrencySymbol; String, fake_currency_symbol_en, fake_currency_symbol_fr, fake_currency_symbol_cn, fake_currency_symbol_tw }

// Filesystem
use fake::faker::filesystem::raw::*;

check_determinism! { l10d DirPath; String, fake_dir_path_en, fake_dir_path_fr, fake_dir_path_cn, fake_dir_path_tw }
check_determinism! { l10d FileExtension; String, fake_file_extension_en, fake_file_extension_fr, fake_file_extension_cn, fake_file_extension_tw }
check_determinism! { l10d FileName; String, fake_file_name_en, fake_file_name_fr, fake_file_name_cn, fake_file_name_tw }
check_determinism! { l10d FilePath; String, fake_file_path_en, fake_file_path_fr, fake_file_path_cn, fake_file_path_tw }
check_determinism! { l10d MimeType; String, fake_mime_type_en, fake_mime_type_fr, fake_mime_type_cn, fake_mime_type_tw }
check_determinism! { l10d Semver; String, fake_semver_en, fake_semver_fr, fake_semver_cn, fake_semver_tw }

// Finance
use fake::faker::finance::raw::*;

check_determinism! { l10d Bic; String, fake_bic_en, fake_bic_fr, fake_bic_cn, fake_bic_tw }

// HTTP
#[cfg(feature = "http")]
mod http {
    use fake::{faker::http::raw::*, Fake, locales::* };
    use rand::SeedableRng as _;

    check_determinism! { l10d RfcStatusCode; String, fake_rfc_status_code_en, fake_rfc_status_code_fr, fake_rfc_status_code_cn, fake_rfc_status_code_tw }
    check_determinism! { l10d ValidStatusCode; String, fake_valid_status_code_en, fake_valid_status_code_fr, fake_valid_status_code_cn, fake_valid_status_code_tw }
}

// Internet
use fake::faker::internet::raw::*;

check_determinism! { l10d Color; String, fake_color_en, fake_color_fr, fake_color_cn, fake_color_tw }
check_determinism! { l10d DomainSuffix; String, fake_domainsuffix_en, fake_domainsuffix_fr, fake_domainsuffix_cn, fake_domainsuffix_tw }
check_determinism! { l10d FreeEmailProvider; String, fake_freeemailprovider_en, fake_freeemailprovider_fr, fake_freeemailprovider_cn, fake_freeemailprovider_tw }
check_determinism! { l10d FreeEmail; String, fake_freeemail_en, fake_freeemail_fr, fake_freeemail_cn, fake_freeemail_tw }
check_determinism! { l10d IP; std::net::IpAddr, fake_ip_en, fake_ip_fr, fake_ip_cn, fake_ip_tw }
check_determinism! { l10d IPv4; String, fake_ipv4_en, fake_ipv4_fr, fake_ipv4_cn, fake_ipv4_tw }
check_determinism! { l10d IPv6; String, fake_ipv6_en, fake_ipv6_fr, fake_ipv6_cn, fake_ipv6_tw }
check_determinism! { l10d MACAddress; String, fake_macaddress_en, fake_macaddress_fr, fake_macaddress_cn, fake_macaddress_tw }
check_determinism! { l10d Password; String, fake_password_en, fake_password_fr, fake_password_cn, fake_password_tw, 6..12 }
check_determinism! { l10d SafeEmail; String, fake_safeemail_en, fake_safeemail_fr, fake_safeemail_cn, fake_safeemail_tw }
check_determinism! { l10d UserAgent; String, fake_useragent_en, fake_useragent_fr, fake_useragent_cn, fake_useragent_tw }
check_determinism! { l10d Username; String, fake_username_en, fake_username_fr, fake_username_cn, fake_username_tw }
// it's sufficient to check one language, because it doesn't change anything
#[cfg(feature = "uuid")]
mod uuid {
    use fake::Fake;
    use fake::uuid::*;
    use rand::SeedableRng as _;
    check_determinism! { one fake_uuid_v1, uuid::Uuid, UUIDv1 }
    check_determinism! { one fake_uuid_v3, uuid::Uuid, UUIDv3 }
    check_determinism! { one fake_uuid_v4, uuid::Uuid, UUIDv4 }
    check_determinism! { one fake_uuid_v5, uuid::Uuid, UUIDv5 }
}

// Job
mod job {
    use fake::{faker::job::raw::*, Fake, locales::* };
    use rand::SeedableRng as _;

    check_determinism! { l10d Field; String, fake_field_en, fake_field_fr, fake_field_cn, fake_field_tw }
    check_determinism! { l10d Position; String, fake_position_en, fake_position_fr, fake_position_cn, fake_position_tw }
    check_determinism! { l10d Seniority; String, fake_seniority_en, fake_seniority_fr, fake_seniority_cn, fake_seniority_tw }
    check_determinism! { l10d Title; String, fake_title_en, fake_title_fr, fake_title_cn, fake_title_tw }
}

// Lorem
use fake::faker::lorem::raw::*;

check_determinism! { l10d Paragraph; String, fake_paragraph_en, fake_paragraph_fr, fake_paragraph_cn, fake_paragraph_tw, 1..3 }
check_determinism! { l10d Paragraphs; Vec<String>, fake_paragraphs_en, fake_paragraphs_fr, fake_paragraphs_cn, fake_paragraphs_tw, 1..3 }
check_determinism! { l10d Sentence; String, fake_sentence_en, fake_sentence_fr, fake_sentence_cn, fake_sentence_tw, (1..3) }
check_determinism! { l10d Sentences; Vec<String>, fake_sentences_en, fake_sentences_fr, fake_sentences_cn, fake_sentences_tw, (1..3) }
check_determinism! { l10d Word; String, fake_word_en, fake_word_fr, fake_word_cn, fake_word_tw }
check_determinism! { l10d Words; Vec<String>, fake_words_en, fake_words_fr, fake_words_cn, fake_words_tw, (1..3) }

// Names
mod name {
    use fake::{faker::name::raw::*, Fake, locales::* };
    use rand::SeedableRng as _;

    check_determinism! { l10d Name; String, fake_name_en, fake_name_fr, fake_name_cn, fake_name_tw }
    check_determinism! { l10d FirstName; String, fake_first_name_en, fake_first_name_fr, fake_first_name_cn, fake_first_name_tw }
    check_determinism! { l10d LastName; String, fake_last_name_en, fake_last_name_fr, fake_last_name_cn, fake_last_name_tw }
    check_determinism! { l10d NameWithTitle; String, fake_name_with_title_en, fake_name_with_title_fr, fake_name_with_title_cn, fake_name_with_title_tw }
    check_determinism! { l10d Title; String, fake_title_en, fake_title_fr, fake_title_cn, fake_title_tw }
    check_determinism! { l10d Suffix; String, fake_suffix_en, fake_suffix_fr, fake_suffix_cn, fake_suffix_tw }
}

// Numbers
use fake::faker::number::raw::*;

check_determinism! { l10d Digit; String, fake_digit_en, fake_digit_fr, fake_digit_cn, fake_digit_tw }
check_determinism! { l10d NumberWithFormat; String, fake_number_en, fake_number_fr, fake_number_cn, fake_number_tw, "{}" }

// Phone Numbers
use fake::faker::phone_number::raw::*;

check_determinism! { l10d CellNumber; String, fake_cell_number_en, fake_cell_number_fr, fake_cell_number_cn, fake_cell_number_tw }
check_determinism! { l10d PhoneNumber; String, fake_phone_number_en, fake_phone_number_fr, fake_phone_number_cn, fake_phone_number_tw }

