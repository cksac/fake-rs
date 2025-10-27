use fake::{locales::*, Fake, Faker};
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
     (l10d $op:expr; $ty:ty, $name_en:ident, $name_fr:ident, $name_cn:ident, $name_tw:ident, $name_ja:ident, $name_br:ident, $name_de:ident, $name_it:ident, $name_nl:ident, $name_cy:ident $(, $attrs:expr)*) => {
        check_determinism!(one $name_en, $ty, $op(EN $(, $attrs)*));
        check_determinism!(one $name_fr, $ty, $op(FR_FR $(, $attrs)*));
        check_determinism!(one $name_cn, $ty, $op(ZH_CN $(, $attrs)*));
        check_determinism!(one $name_tw, $ty, $op(ZH_TW $(, $attrs)*));
        check_determinism!(one $name_ja, $ty, $op(JA_JP $(, $attrs)*));
        check_determinism!(one $name_br, $ty, $op(PT_BR $(, $attrs)*));
        check_determinism!(one $name_de, $ty, $op(DE_DE $(, $attrs)*));
        check_determinism!(one $name_it, $ty, $op(IT_IT $(, $attrs)*));
        check_determinism!(one $name_cy, $ty, $op(CY_GB $(, $attrs)*));
        check_determinism!(one $name_nl, $ty, $op(NL_NL $(, $attrs)*));
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

check_determinism! { l10d BuildingNumber; String, fake_buildingnumber_en, fake_buildingnumber_fr, fake_buildingnumber_cn, fake_buildingnumber_tw, fake_buildingnumber_jp, fake_buildingnumber_br, fake_buildingnumber_de, fake_buildingnumber_it, fake_buildingnumber_cy, fake_buildingnumber_nl}
check_determinism! { l10d CityName; String, fake_cityname_en, fake_cityname_fr, fake_cityname_cn, fake_cityname_tw, fake_cityname_jp, fake_cityname_br, fake_cityname_de, fake_cityname_it, fake_cityname_cy, fake_cityname_nl}
check_determinism! { l10d CityPrefix; String, fake_cityprefix_en, fake_cityprefix_fr, fake_cityprefix_cn, fake_cityprefix_tw, fake_cityprefix_jp, fake_cityprefix_br, fake_cityprefix_de, fake_cityprefix_it, fake_cityprefix_cy, fake_cityprefix_nl}
check_determinism! { l10d CitySuffix; String, fake_citysuffix_en, fake_citysuffix_fr, fake_citysuffix_cn, fake_citysuffix_tw, fake_citysuffix_jp, fake_citysuffix_br, fake_citysuffix_de, fake_citysuffix_it, fake_citysuffix_cy, fake_citysuffix_nl }
check_determinism! { l10d CountryCode; String, fake_countrycode_en, fake_countrycode_fr, fake_countrycode_cn, fake_countrycode_tw, fake_countrycode_jp, fake_countrycode_br, fake_countrycode_de, fake_countrycode_it, fake_countrycode_cy, fake_countrycode_nl}
check_determinism! { l10d CountryName; String, fake_countryname_en, fake_countryname_fr, fake_countryname_cn, fake_countryname_tw, fake_countryname_jp, fake_countryname_br, fake_countryname_de, fake_countryname_it, fake_countryname_cy, fake_countryname_nl}
check_determinism! { l10d Geohash; String, fake_geohash_en, fake_geohash_fr, fake_geohash_cn, fake_geohash_tw, fake_geohash_jp,fake_geohash_br, fake_geohash_de, fake_geohash_it, fake_geohash_cy, fake_geohash_nl, 11 }
check_determinism! { l10d Latitude; String, fake_latitude_en, fake_latitude_fr, fake_latitude_cn, fake_latitude_tw, fake_latitude_jp, fake_latitude_br, fake_latitude_de, fake_latitude_it, fake_latitude_cy, fake_latitude_nl }
check_determinism! { l10d Longitude; String, fake_longitude_en, fake_longitude_fr, fake_longitude_cn, fake_longitude_tw, fake_longitude_jp, fake_longitude_br, fake_longitude_de, fake_longitude_it, fake_longitude_cy, fake_longitude_nl }
check_determinism! { l10d PostCode; String, fake_postcode_en, fake_postcode_fr, fake_postcode_cn, fake_postcode_tw, fake_postcode_jp, fake_postcode_br, fake_postcode_de, fake_postcode_it, fake_postcode_cy, fake_postcode_nl }
check_determinism! { l10d SecondaryAddress; String, fake_secondary_address_en, fake_secondary_address_fr, fake_secondary_address_cn, fake_secondary_address_tw, fake_secondary_address_jp, fake_secondary_address_br, fake_secondary_address_de, fake_secondary_address_it, fake_secondary_address_cy, fake_secondary_address_nl }
check_determinism! { l10d SecondaryAddressType; String, fake_secondary_address_type_en, fake_secondary_address_type_fr, fake_secondary_address_type_cn, fake_secondary_address_type_tw, fake_secondary_address_type_jp, fake_secondary_address_type_br, fake_secondary_address_type_de, fake_secondary_address_type_it, fake_secondary_address_type_cy, fake_secondary_address_type_nl }
check_determinism! { l10d StateAbbr; String, fake_state_abbr_en, fake_state_abbr_fr, fake_state_abbr_cn, fake_state_abbr_tw, fake_state_abbr_jp, fake_state_abbr_br, fake_state_abbr_de, fake_state_abbr_it, fake_state_abbr_cy, fake_state_abbr_nl }
check_determinism! { l10d StateName; String, fake_state_name_en, fake_state_name_fr, fake_state_name_cn, fake_state_name_tw, fake_state_name_jp, fake_state_name_br, fake_state_name_de, fake_state_name_it, fake_state_name_cy, fake_state_name_nl }
check_determinism! { l10d StreetName; String, fake_street_name_en, fake_street_name_fr, fake_street_name_cn, fake_street_name_tw, fake_street_name_jp, fake_street_name_br, fake_street_name_de, fake_street_name_it, fake_street_name_cy, fake_street_name_nl }
check_determinism! { l10d StreetSuffix; String, fake_street_suffix_en, fake_street_suffix_fr, fake_street_suffix_cn, fake_street_suffix_tw, fake_street_suffix_jp, fake_street_suffix_br, fake_street_suffix_de, fake_street_suffix_it, fake_street_suffix_cy, fake_street_suffix_nl }
check_determinism! { l10d TimeZone; String, fake_time_zone_en, fake_time_zone_fr, fake_time_zone_cn, fake_time_zone_tw, fake_time_zone_jp, fake_time_zone_br, fake_time_zone_de, fake_time_zone_it, fake_time_zone_cy, fake_time_zone_nl }
check_determinism! { l10d ZipCode; String, fake_zip_code_en, fake_zip_code_fr, fake_zip_code_cn, fake_zip_code_tw, fake_zip_code_jp, fake_zip_code_br, fake_zip_code_de, fake_zip_code_it, fake_zip_code_cy, fake_zip_code_nl }

// Administrative
use fake::faker::administrative::raw::*;

check_determinism! { one fake_health_insurance_code_fr, String, HealthInsuranceCode(FR_FR) }

// Automotive
use fake::faker::automotive::raw::*;

check_determinism! { one fake_license_plate_fr, String, LicencePlate(FR_FR) }
check_determinism! { one fake_license_plate_it, String, LicencePlate(IT_IT) }

// Barcode
use fake::faker::barcode::raw::*;

check_determinism! { one fake_isbn_en, String, Isbn(EN) }
check_determinism! { one fake_isbn10_en, String, Isbn10(EN) }
check_determinism! { one fake_isbn13_en, String, Isbn13(EN) }

// Color
#[cfg(feature = "random_color")]
mod color {
    use fake::{faker::color::raw::*, locales::*, Fake};
    use rand::SeedableRng as _;

    check_determinism! { one fake_hexcolor_en, String, HexColor(EN) }
    check_determinism! { one fake_rgbcolor_en, String, RgbColor(EN) }
    check_determinism! { one fake_rgbacolor_en, String, RgbaColor(EN) }
    check_determinism! { one fake_hslcolor_en, String, HslColor(EN) }
    check_determinism! { one fake_hslacolor_en, String, HslaColor(EN) }
    check_determinism! { one fake_color_en, String, Color(EN) }
}

// Chrono
#[cfg(feature = "chrono")]
mod chrono {
    use fake::{chrono::Precision, faker::chrono::raw::*, locales::*, Fake};
    use rand::SeedableRng as _;

    fn lo() -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp(53469346924, 124241).expect("datetime")
    }

    fn hi() -> chrono::DateTime<chrono::Utc> {
        lo() + chrono::Duration::days(365)
    }

    check_determinism! { l10d Date; String, fake_date_en, fake_date_fr, fake_date_cn, fake_date_tw, fake_date_jp, fake_date_br, fake_date_de, fake_date_it, fake_date_cy,fake_date_nl}
    check_determinism! { l10d DateTime; String, fake_date_time_en, fake_date_time_fr, fake_date_time_cn, fake_date_time_tw, fake_date_time_jp, fake_date_time_br, fake_date_time_de, fake_date_time_it, fake_date_time_cy, fake_date_time_nl}
    check_determinism! { l10d DateTimeAfter; String, fake_date_time_after_en, fake_date_time_after_fr, fake_date_time_after_cn, fake_date_time_after_tw, fake_date_time_after_jp, fake_date_time_after_br, fake_date_time_after_de, fake_date_time_after_it, fake_date_time_after_cy, fake_date_time_after_nl, lo()}
    check_determinism! { l10d DateTimeBefore; String, fake_date_time_before_en, fake_date_time_before_fr, fake_date_time_before_cn, fake_date_time_before_tw, fake_date_time_before_jp, fake_date_time_before_br, fake_date_time_before_de, fake_date_time_before_it, fake_date_time_before_cy, fake_date_time_before_nl, hi()}
    check_determinism! { l10d DateTimeBetween; String, fake_date_time_between_en, fake_date_time_between_fr, fake_date_time_between_cn, fake_date_time_between_tw, fake_date_time_between_jp, fake_date_time_between_br, fake_date_time_between_de, fake_date_time_between_it, fake_date_time_between_cy, fake_date_time_between_nl, lo(), hi()}
    check_determinism! { l10d Duration; ::chrono::Duration, fake_duration_en, fake_duration_fr, fake_duration_cn, fake_duration_tw, fake_duration_jp, fake_duration_br, fake_duration_de, fake_duration_it, fake_duration_cy, fake_duration_nl}
    check_determinism! { l10d Time; String, fake_time_en, fake_time_fr, fake_time_cn, fake_time_tw, fake_time_jp, fake_time_br, fake_time_de, fake_time_it, fake_time_cy, fake_time_nl}
    check_determinism! { one fake_naive_time_precision, ::chrono::NaiveTime, Precision::<6> }
    check_determinism! { one fake_naive_date_time_precision, ::chrono::NaiveDateTime, Precision::<6> }
    check_determinism! { one fake_date_time_precision, ::chrono::DateTime<::chrono::Utc>, Precision::<6> }
}

// time
#[cfg(feature = "time")]
mod time {
    use fake::{faker::time::raw::*, locales::*, Fake};
    use rand::SeedableRng as _;

    fn lo() -> time::OffsetDateTime {
        time::OffsetDateTime::from_unix_timestamp(53469346924).unwrap()
    }

    fn hi() -> time::OffsetDateTime {
        lo() + time::Duration::days(365)
    }

    check_determinism! { l10d Date; String, fake_date_en, fake_date_fr, fake_date_cn, fake_date_tw, fake_date_jp, fake_date_br, fake_date_de, fake_date_it, fake_date_cy, fake_date_nl }
    check_determinism! { l10d DateTime; String, fake_date_time_en, fake_date_time_fr, fake_date_time_cn, fake_date_time_tw, fake_date_time_jp, fake_date_time_br, fake_date_time_de, fake_date_time_it, fake_date_time_cy, fake_date_time_nl}
    check_determinism! { l10d DateTimeAfter; String, fake_date_time_after_en, fake_date_time_after_fr, fake_date_time_after_cn, fake_date_time_after_tw, fake_date_time_after_jp, fake_date_time_after_br, fake_date_time_after_de, fake_date_time_after_it, fake_date_time_after_cy, fake_date_time_after_nl, lo() }
    check_determinism! { l10d DateTimeBefore; String, fake_date_time_before_en, fake_date_time_before_fr, fake_date_time_before_cn, fake_date_time_before_tw, fake_date_time_before_jp, fake_date_time_before_br, fake_date_time_before_de, fake_date_time_before_it, fake_date_time_before_cy, fake_date_time_before_nl, hi() }
    check_determinism! { l10d DateTimeBetween; String, fake_date_time_between_en, fake_date_time_between_fr, fake_date_time_between_cn, fake_date_time_between_tw, fake_date_time_between_jp, fake_date_time_between_br, fake_date_time_between_de, fake_date_time_between_it, fake_date_time_between_cy, fake_date_time_between_nl, lo(), hi() }
    check_determinism! { l10d Duration; ::time::Duration, fake_duration_en, fake_duration_fr, fake_duration_cn, fake_duration_tw, fake_duration_jp, fake_duration_br, fake_duration_de, fake_duration_it, fake_duration_cy, fake_duration_nl }
    check_determinism! { l10d Time; String, fake_time_en, fake_time_fr, fake_time_cn, fake_time_tw, fake_time_jp, fake_time_br, fake_time_de, fake_time_it, fake_time_cy, fake_time_nl }
}

// Company
use fake::faker::company::raw::*;

check_determinism! { l10d Bs; String, fake_bs_en, fake_bs_fr, fake_bs_cn, fake_bs_tw, fake_bs_jp, fake_bs_br, fake_bs_de, fake_bs_it, fake_bs_cy, fake_bs_nl }
check_determinism! { l10d BsAdj; String, fake_bs_adj_en, fake_bs_adj_fr, fake_bs_adj_cn, fake_bs_adj_tw, fake_bs_adj_jp, fake_bs_adj_br, fake_bs_adj_de, fake_bs_adj_it, fake_bs_adj_cy, fake_bs_adj_nl }
check_determinism! { l10d BsNoun; String, fake_bs_noun_en, fake_bs_noun_fr, fake_bs_noun_cn, fake_bs_noun_tw, fake_bs_noun_jp, fake_bs_noun_br,fake_bs_noun_de, fake_bs_noun_it, fake_bs_noun_cy, fake_bs_noun_nl }
check_determinism! { l10d BsVerb; String, fake_bs_verb_en, fake_bs_verb_fr, fake_bs_verb_cn, fake_bs_verb_tw, fake_bs_verb_jp, fake_bs_verb_br, fake_bs_verb_de, fake_bs_verb_it, fake_bs_verb_cy, fake_bs_verb_nl }
check_determinism! { l10d Buzzword; String, fake_buzzword_en, fake_buzzword_fr, fake_buzzword_cn, fake_buzzword_tw, fake_buzzword_jp, fake_buzzword_br, fake_buzzword_de, fake_buzzword_it, fake_buzzword_cy, fake_buzzword_nl }
check_determinism! { l10d BuzzwordMiddle; String, fake_buzzword_middle_en, fake_buzzword_middle_fr, fake_buzzword_middle_cn, fake_buzzword_middle_tw, fake_buzzword_middle_jp, fake_buzzword_middle_br,fake_buzzword_middle_de, fake_buzzword_middle_it, fake_buzzword_middle_cy, fake_buzzword_middle_nl }
check_determinism! { l10d BuzzwordTail; String, fake_buzzword_tail_en, fake_buzzword_tail_fr, fake_buzzword_tail_cn, fake_buzzword_tail_tw, fake_buzzword_tail_jp, fake_buzzword_tail_br, fake_buzzword_tail_de, fake_buzzword_tail_it, fake_buzzword_tail_cy, fake_buzzword_tail_nl }
check_determinism! { l10d CatchPhrase; String, fake_catchphrase_en, fake_catchphrase_fr, fake_catchphrase_cn, fake_catchphrase_tw, fake_catchphrase_jp, fake_catchphrase_br, fake_catchphrase_de, fake_catchphrase_it, fake_catchphrase_cy, fake_catchphrase_nl }
check_determinism! { l10d CompanyName; String, fake_company_name_en, fake_company_name_fr, fake_company_name_cn, fake_company_name_tw, fake_company_name_jp, fake_company_name_br, fake_company_name_de, fake_company_name_it, fake_company_name_cy, fake_company_name_nl }
check_determinism! { l10d CompanySuffix; String, fake_company_suffix_en, fake_company_suffix_fr, fake_company_suffix_cn, fake_company_suffix_tw, fake_company_suffix_jp, fake_company_suffix_br, fake_company_suffix_de, fake_company_suffix_it, fake_company_suffix_cy, fake_company_suffix_nl }
check_determinism! { l10d Industry; String, fake_industry_en, fake_industry_fr, fake_industry_cn, fake_industry_tw, fake_industry_jp, fake_industry_br, fake_industry_de, fake_industry_it, fake_industry_cy, fake_industry_nl }
check_determinism! { l10d Profession; String, fake_profession_en, fake_profession_fr, fake_profession_cn, fake_profession_tw, fake_profession_jp, fake_profession_br, fake_profession_de, fake_profession_it, fake_profession_cy, fake_profession_nl }

// Credit Card
use fake::faker::creditcard::raw::*;

// it's sufficient to check one language, because it doesn't change anything
check_determinism! { one fake_credit_card_number_en, String, CreditCardNumber(EN) }

// Currency
use fake::faker::currency::raw::*;

check_determinism! { l10d CurrencyCode; String, fake_currency_code_en, fake_currency_code_fr, fake_currency_code_cn, fake_currency_code_tw, fake_currency_code_jp, fake_currency_code_br, fake_currency_code_de, fake_currency_code_it, fake_currency_code_cy, fake_currency_cde_nl }
check_determinism! { l10d CurrencyName; String, fake_currency_name_en, fake_currency_name_fr, fake_currency_name_cn, fake_currency_name_tw, fake_currency_name_jp, fake_currency_name_br, fake_currency_name_de, fake_currency_name_it, fake_currency_name_cy, fake_currency_name_nl }
check_determinism! { l10d CurrencySymbol; String, fake_currency_symbol_en, fake_currency_symbol_fr, fake_currency_symbol_cn, fake_currency_symbol_tw, fake_currency_symbol_jp, fake_currency_symbol_br, fake_currency_symbol_de, fake_currency_symbol_it, fake_currency_symbol_cy, fake_currency_symbol_nl }

// Filesystem
use fake::faker::filesystem::raw::*;

check_determinism! { one fake_dir_path_en, String, DirPath(EN) }
check_determinism! { one fake_file_extension_en, String, FileExtension(EN) }
check_determinism! { one fake_file_name_en, String, FileName(EN) }
check_determinism! { one fake_file_path_en, String, FilePath(EN) }
check_determinism! { one fake_mime_type_en, String, MimeType(EN) }
check_determinism! { one fake_semver_en, String, Semver(EN) }

// Finance
use fake::faker::finance::raw::*;

check_determinism! { l10d Bic; String, fake_bic_en, fake_bic_fr, fake_bic_cn, fake_bic_tw, fake_bic_jp, fake_bic_br, fake_bic_de, fake_bic_it, fake_bic_cy, fake_bic_nl }
check_determinism! { l10d Isin; String, fake_isin_en, fake_isin_fr, fake_isin_cn, fake_isin_tw, fake_isin_jp, fake_isin_br, fake_isin_de, fake_isin_it, fake_isin_cy, fake_isin_nl }

// HTTP
#[cfg(feature = "http")]
mod http {
    use fake::{faker::http::raw::*, locales::*, Fake};
    use rand::SeedableRng as _;

    check_determinism! { one fake_rfc_status_code_en, String, RfcStatusCode(EN) }
    check_determinism! { one fake_valid_status_code_en, String, ValidStatusCode(EN) }
}

// Internet
use fake::faker::internet::raw::*;

check_determinism! { l10d DomainSuffix; String, fake_domainsuffix_en, fake_domainsuffix_fr, fake_domainsuffix_cn, fake_domainsuffix_tw, fake_domainsuffix_jp, fake_domainsuffix_br, fake_domainsuffix_de, fake_domainsuffix_it, fake_domainsuffix_cy, fake_domainsuffix_nl }
check_determinism! { l10d FreeEmailProvider; String, fake_freeemailprovider_en, fake_freeemailprovider_fr, fake_freeemailprovider_cn, fake_freeemailprovider_tw, fake_freeemailprovider_jp, fake_freeemailprovider_br, fake_freeemailprovider_de, fake_freeemailprovider_it, fake_freeemailprovider_cy, fake_feeemailprovider_nl }
check_determinism! { l10d FreeEmail; String, fake_freeemail_en, fake_freeemail_fr, fake_freeemail_cn, fake_freeemail_tw, fake_freeemail_jp, fake_freeemail_br, fake_freeemail_de, fake_freeemail_it, fake_freeemail_cy, fake_feeemail_nl }
check_determinism! { l10d IP; std::net::IpAddr, fake_ip_en, fake_ip_fr, fake_ip_cn, fake_ip_tw, fake_ip_jp, fake_ip_br, fake_ip_de, fake_ip_it, fake_ip_cy, fake_ip_nl }
check_determinism! { l10d IPv4; String, fake_ipv4_en, fake_ipv4_fr, fake_ipv4_cn, fake_ipv4_tw, fake_ipv4_jp, fake_ipv4_br, fake_ipv4_de, fake_ipv4_it, fake_ipv4_cy, fake_ipv4_nl }
check_determinism! { l10d IPv6; String, fake_ipv6_en, fake_ipv6_fr, fake_ipv6_cn, fake_ipv6_tw, fake_ipv6_jp, fake_ipv6_br, fake_ipv6_de, fake_ipv6_it, fake_ipv6_cy, fake_ipv6_nl }
check_determinism! { l10d MACAddress; String, fake_macaddress_en, fake_macaddress_fr, fake_macaddress_cn, fake_macaddress_tw, fake_macaddress_jp, fake_macaddress_br, fake_macaddress_de, fake_macaddress_it, fake_macaddress_cy, fake_macaddress_nl }
check_determinism! { l10d Password; String, fake_password_en, fake_password_fr, fake_password_cn, fake_password_tw, fake_password_jp, fake_password_br, fake_password_de, fake_password_it, fake_password_cy, fake_password_nl, 6..12 }
check_determinism! { l10d SafeEmail; String, fake_safeemail_en, fake_safeemail_fr, fake_safeemail_cn, fake_safeemail_tw, fake_safeemail_jp, fake_safeemail_br, fake_safeemail_de, fake_safeemail_it, fake_safeemail_cy, fake_safeemail_nl }
check_determinism! { l10d UserAgent; String, fake_useragent_en, fake_useragent_fr, fake_useragent_cn, fake_useragent_tw, fake_useragent_jp, fake_useragent_br, fake_useragent_de, fake_useragent_it, fake_useragent_cy, fake_useragent_nl }
check_determinism! { l10d Username; String, fake_username_en, fake_username_fr, fake_username_cn, fake_username_tw, fake_username_jp, fake_username_br, fake_username_de, fake_username_it, fake_username_cy, fake_username_nl }

// it's sufficient to check one language, because it doesn't change anything
#[cfg(feature = "ulid")]
mod ulid {
    use fake::{Fake, Faker};
    use rand::SeedableRng as _;
    check_determinism! { one fake_ulid, ulid::Ulid, Faker }
}

// it's sufficient to check one language, because it doesn't change anything
#[cfg(feature = "ferroid")]
mod ferroid {
    use fake::ferroid::*;
    use fake::{Fake, Faker};
    use rand::SeedableRng as _;
    check_determinism! { one fake_ferroid_ulid_f, ferroid::id::ULID, Faker }
    check_determinism! { one fake_ferroid_ulid, ferroid::id::ULID, FerroidULID }
    check_determinism! { one fake_ferroid_snowflake_twitter_f, ferroid::id::SnowflakeTwitterId, Faker }
    check_determinism! { one fake_ferroid_snowflake_twitter, ferroid::id::SnowflakeTwitterId, FerroidTwitterId }
    check_determinism! { one fake_ferroid_snowflake_mastodon_f, ferroid::id::SnowflakeMastodonId, Faker }
    check_determinism! { one fake_ferroid_snowflake_mastodon, ferroid::id::SnowflakeMastodonId, FerroidMastodonId }
    check_determinism! { one fake_ferroid_snowflake_discord_f, ferroid::id::SnowflakeDiscordId, Faker }
    check_determinism! { one fake_ferroid_snowflake_discord, ferroid::id::SnowflakeDiscordId, FerroidDiscordId }
    check_determinism! { one fake_ferroid_snowflake_instagram_f, ferroid::id::SnowflakeInstagramId, Faker }
    check_determinism! { one fake_ferroid_snowflake_instagram, ferroid::id::SnowflakeInstagramId, FerroidInstagramId }
}

// it's sufficient to check one language, because it doesn't change anything
#[cfg(feature = "uuid")]
mod uuid {
    use fake::uuid::*;
    use fake::Fake;
    use rand::SeedableRng as _;
    check_determinism! { one fake_uuid_v1, uuid::Uuid, UUIDv1 }
    check_determinism! { one fake_uuid_v3, uuid::Uuid, UUIDv3 }
    check_determinism! { one fake_uuid_v4, uuid::Uuid, UUIDv4 }
    check_determinism! { one fake_uuid_v5, uuid::Uuid, UUIDv5 }
}

// Job
mod job {
    use fake::{faker::job::raw::*, locales::*, Fake};
    use rand::SeedableRng as _;

    check_determinism! { l10d Field; String, fake_field_en, fake_field_fr, fake_field_cn, fake_field_tw, fake_field_jp, fake_field_br,fake_field_de, fake_field_it, fake_field_cy, fake_field_nl }
    check_determinism! { l10d Position; String, fake_position_en, fake_position_fr, fake_position_cn, fake_position_tw, fake_position_jp, fake_position_br, fake_position_de, fake_position_it, fake_position_cy, fake_position_nl }
    check_determinism! { l10d Seniority; String, fake_seniority_en, fake_seniority_fr, fake_seniority_cn, fake_seniority_tw, fake_seniority_jp, fake_seniority_br, fake_seniority_de, fake_seniority_it, fake_seniority_cy, fake_seniority_nl }
    check_determinism! { l10d Title; String, fake_title_en, fake_title_fr, fake_title_cn, fake_title_tw, fake_title_jp, fake_title_br, fake_title_de, fake_title_it, fake_title_cy, fake_title_nl }
}

// Lorem
use fake::faker::lorem::raw::*;

check_determinism! { l10d Paragraph; String, fake_paragraph_en, fake_paragraph_fr, fake_paragraph_cn, fake_paragraph_tw, fake_paragraph_jp, fake_paragraph_br, fake_paragraph_de, fake_paragraph_it, fake_paragraph_cy, fake_paragraph_nl, 1..3 }
check_determinism! { l10d Paragraphs; Vec<String>, fake_paragraphs_en, fake_paragraphs_fr, fake_paragraphs_cn, fake_paragraphs_tw, fake_paragraphs_jp, fake_paragraphs_br, fake_paragraphs_de, fake_paragraphs_it, fake_paragraphs_cy, fake_paragraphs_nl, 1..3 }
check_determinism! { l10d Sentence; String, fake_sentence_en, fake_sentence_fr, fake_sentence_cn, fake_sentence_tw, fake_sentence_jp, fake_sentence_br, fake_sentence_de, fake_sentence_it, fake_sentence_cy, fake_sentence_nl, (1..3) }
check_determinism! { l10d Sentences; Vec<String>, fake_sentences_en, fake_sentences_fr, fake_sentences_cn, fake_sentences_tw, fake_sentences_jp, fake_sentences_br, fake_sentences_de, fake_sentences_it, fake_sentences_cy, fake_sentences_nl, (1..3) }
check_determinism! { l10d Word; String, fake_word_en, fake_word_fr, fake_word_cn, fake_word_tw, fake_word_jp, fake_word_br, fake_word_de, fake_word_it, fake_word_cy, fake_word_nl }
check_determinism! { l10d Words; Vec<String>, fake_words_en, fake_words_fr, fake_words_cn, fake_words_tw, fake_words_jp, fake_words_br, fake_words_de, fake_words_it, fake_words_cy, fake_words_nl, (1..3) }

// Names
mod name {
    use fake::{faker::name::raw::*, locales::*, Fake};
    use rand::SeedableRng as _;

    check_determinism! { l10d Name; String, fake_name_en, fake_name_fr, fake_name_cn, fake_name_tw, fake_name_jp, fake_name_br, fake_name_de, fake_name_it, fake_name_cy, fake_name_nl }
    check_determinism! { l10d FirstName; String, fake_first_name_en, fake_first_name_fr, fake_first_name_cn, fake_first_name_tw, fake_first_name_jp, fake_first_name_br, fake_first_name_de, fake_first_name_it, fake_first_name_cy, fake_first_name_nl }
    check_determinism! { l10d LastName; String, fake_last_name_en, fake_last_name_fr, fake_last_name_cn, fake_last_name_tw, fake_last_name_jp, fake_last_name_br , fake_last_name_de, fake_last_name_it, fake_last_name_cy, fake_last_name_nl }
    check_determinism! { l10d NameWithTitle; String, fake_name_with_title_en, fake_name_with_title_fr, fake_name_with_title_cn, fake_name_with_title_tw, fake_name_with_title_jp, fake_name_with_title_br, fake_name_with_title_de, fake_name_with_title_it, fake_name_with_title_cy, fake_name_with_title_nl }
    check_determinism! { l10d Title; String, fake_title_en, fake_title_fr, fake_title_cn, fake_title_tw, fake_title_jp, fake_title_br, fake_title_de, fake_title_it, fake_title_cy, fake_title_nl }
    check_determinism! { l10d Suffix; String, fake_suffix_en, fake_suffix_fr, fake_suffix_cn, fake_suffix_tw, fake_suffix_jp, fake_suffix_br, fake_suffix_jde, fake_suffix_it, fake_suffix_cy, fake_suffix_nl }
}

// Numbers
use fake::faker::number::raw::*;

check_determinism! { l10d Digit; String, fake_digit_en, fake_digit_fr, fake_digit_cn, fake_digit_tw, fake_digit_jp, fake_digit_br,fake_digit_de, fake_digit_it, fake_digit_cy, fake_digit_nl }
check_determinism! { l10d NumberWithFormat; String, fake_number_en, fake_number_fr, fake_number_cn, fake_number_tw, fake_number_jp, fake_number_br, fake_number_de, fake_number_it, fake_number_cy, fake_number_nl, "{}" }

// Phone Numbers
use fake::faker::phone_number::raw::*;

check_determinism! { l10d CellNumber; String, fake_cell_number_en, fake_cell_number_fr, fake_cell_number_cn, fake_cell_number_tw, fake_cell_number_jp, fake_cell_number_br, fake_cell_number_de, fake_cell_number_it, fake_cell_number_cy, fake_cell_number_nl }
check_determinism! { l10d PhoneNumber; String, fake_phone_number_en, fake_phone_number_fr, fake_phone_number_cn, fake_phone_number_tw, fake_phone_number_jp, fake_phone_number_br, fake_phone_number_de, fake_phone_number_it, fake_phone_number_cy, fake_phone_number_nl }

#[cfg(feature = "geo-types")]
mod geo {
    use fake::{Fake, Faker};
    use rand::SeedableRng as _;

    check_determinism! { one fake_geo_coord_f64, geo_types::Coord<f64>, Faker }
    check_determinism! { one fake_geo_coord_u64, geo_types::Coord<u64>, Faker }

    check_determinism! { one fake_geo_line_f64, geo_types::Line<f64>, Faker }
    check_determinism! { one fake_geo_line_u64, geo_types::Line<u64>, Faker }

    check_determinism! { one fake_geo_linestring_f64, geo_types::LineString<f64>, Faker }
    check_determinism! { one fake_geo_linestring_u64, geo_types::LineString<u64>, Faker }

    check_determinism! { one fake_geo_multilinestring_f64, geo_types::MultiLineString<f64>, Faker }
    check_determinism! { one fake_geo_multilinestring_u64, geo_types::MultiLineString<u64>, Faker }

    check_determinism! { one fake_geo_point_f64, geo_types::Point<f64>, Faker }
    check_determinism! { one fake_geo_point_u64, geo_types::Point<u64>, Faker }

    check_determinism! { one fake_geo_multipoint_f64, geo_types::MultiPoint<f64>, Faker }
    check_determinism! { one fake_geo_multipoint_u64, geo_types::MultiPoint<u64>, Faker }

    check_determinism! { one fake_geo_multipolygon_f64, geo_types::MultiPolygon<f64>, Faker }
    check_determinism! { one fake_geo_multipolygon_u64, geo_types::MultiPolygon<u64>, Faker }

    check_determinism! { one fake_geo_polygon_f64, geo_types::Polygon<f64>, Faker }
    check_determinism! { one fake_geo_polygon_u64, geo_types::Polygon<u64>, Faker }

    check_determinism! { one fake_geo_rect_f64, geo_types::Rect<f64>, Faker }
    check_determinism! { one fake_geo_rect_u64, geo_types::Rect<u64>, Faker }

    check_determinism! { one fake_geo_triangle_f64, geo_types::Triangle<f64>, Faker }
    check_determinism! { one fake_geo_triangle_u64, geo_types::Triangle<u64>, Faker }

    check_determinism! { one fake_geo_geometry_f64, geo_types::Geometry<f64>, Faker }
    check_determinism! { one fake_geo_geometry_u64, geo_types::Geometry<u64>, Faker }

    check_determinism! { one fake_geo_geometry_collection_f64, geo_types::GeometryCollection<f64>, Faker }
    check_determinism! { one fake_geo_geometry_collection_u64, geo_types::GeometryCollection<u64>, Faker }
}

// Decimal
#[cfg(feature = "rust_decimal")]
mod decimal {
    use fake::decimal::*;
    use fake::Fake;
    use rand::SeedableRng as _;
    use rust_decimal as rs;

    check_determinism! { Decimal; default, rs::Decimal }
    check_determinism! { NegativeDecimal; negative_decimal, rs::Decimal }
    check_determinism! { PositiveDecimal; positive_decimal, rs::Decimal }
    check_determinism! { NoDecimalPoints; no_decimal_points, rs::Decimal }
}

// BigDecimal
#[cfg(feature = "bigdecimal")]
mod bigdecimal {
    use bigdecimal_rs as bd;
    use fake::bigdecimal::*;
    use fake::Fake;
    use rand::SeedableRng as _;

    check_determinism! { BigDecimal; default, bd::BigDecimal }
    check_determinism! { NegativeBigDecimal; negative_decimal, bd::BigDecimal }
    check_determinism! { PositiveBigDecimal; positive_decimal, bd::BigDecimal }
    check_determinism! { NoBigDecimalPoints; no_decimal_points, bd::BigDecimal }
}

#[cfg(feature = "serde_json")]
mod serde_json {
    use fake::{Fake, Faker};
    use rand::SeedableRng as _;
    check_determinism! { one fake_serde_json, serde_json::Value, Faker }
}

// BSON ObjectId
#[cfg(feature = "bson_oid")]
mod bson_oid {
    use bson::oid::ObjectId;
    use fake::{Fake, Faker};
    use rand::SeedableRng as _;

    check_determinism! { one fake_bson_oid, ObjectId, Faker }
}

#[cfg(feature = "base64")]
mod base64 {
    use fake::base64::*;
    use fake::{Fake, Faker};
    use rand::SeedableRng as _;

    check_determinism! { one fake_base64, Base64Value, Faker }
    check_determinism! { one fake_url_safe_base64, UrlSafeBase64Value, Faker }
}
