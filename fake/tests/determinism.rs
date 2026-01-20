//! Determinism tests with centralized locale management.
//!
//! This module uses declarative macros with the `paste` crate to reduce
//! maintenance effort when adding new locales - just add the locale to the
//! `for_all_locales!` macro and all localized tests will be automatically generated.
//!
//! ## How to add a new locale:
//! 1. Add the new locale identifier to the `for_all_locales!` macro definition
//! 2. Run `cargo test` - all tests will be auto-generated

use fake::{locales::*, Fake, Faker};
use rand::SeedableRng as _;

/// Helper macro to generate a single determinism test
macro_rules! determinism_test {
    ($name:ident, $ty:ty, $op:expr) => {
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
}

/// Central locale list - ADD NEW LOCALES HERE!
/// When a new locale is added, just add it to this list and all localized tests
/// will be automatically generated.
macro_rules! for_all_locales {
    ($macro_name:ident ! ( $($args:tt)* )) => {
        $macro_name!($($args)* EN);
        $macro_name!($($args)* FR_FR);
        $macro_name!($($args)* ZH_CN);
        $macro_name!($($args)* ZH_TW);
        $macro_name!($($args)* JA_JP);
        $macro_name!($($args)* PT_BR);
        $macro_name!($($args)* DE_DE);
        $macro_name!($($args)* IT_IT);
        $macro_name!($($args)* CY_GB);
        $macro_name!($($args)* NL_NL);
        $macro_name!($($args)* TR_TR);
    };
}

/// Generate determinism tests for a faker with String output type
macro_rules! gen_l10d_string_test {
    ($faker:ident, $locale:ident) => {
        paste::paste! {
            determinism_test!([<fake_ $faker:lower _ $locale:lower>], String, $faker($locale));
        }
    };
}

/// Generate determinism tests for a faker with custom output type
macro_rules! gen_l10d_test {
    ($faker:ident, $ty:ty, $locale:ident) => {
        paste::paste! {
            determinism_test!([<fake_ $faker:lower _ $locale:lower>], $ty, $faker($locale));
        }
    };
}

/// Generate determinism tests for a faker with one extra argument
macro_rules! gen_l10d_string_test_with_arg {
    ($faker:ident, $arg:expr, $locale:ident) => {
        paste::paste! {
            determinism_test!([<fake_ $faker:lower _ $locale:lower>], String, $faker($locale, $arg));
        }
    };
}

/// Generate determinism tests for a faker with custom type and one argument
macro_rules! gen_l10d_test_with_arg {
    ($faker:ident, $ty:ty, $arg:expr, $locale:ident) => {
        paste::paste! {
            determinism_test!([<fake_ $faker:lower _ $locale:lower>], $ty, $faker($locale, $arg));
        }
    };
}

// ============================================================================
// Basic type tests (locale-independent)
// ============================================================================

determinism_test!(check_bool, bool, Faker);
determinism_test!(check_u8, u8, Faker);
determinism_test!(check_u16, u16, Faker);
determinism_test!(check_u32, u32, Faker);
determinism_test!(check_u64, u64, Faker);
determinism_test!(check_u128, u128, Faker);
determinism_test!(check_usize, usize, Faker);
determinism_test!(check_i8, i8, Faker);
determinism_test!(check_i16, i16, Faker);
determinism_test!(check_i32, i32, Faker);
determinism_test!(check_i64, i64, Faker);
determinism_test!(check_i128, i128, Faker);
determinism_test!(check_isize, isize, Faker);
determinism_test!(fake_u64_range, u64, (1u64..54683546546434));

// ============================================================================
// Address
// ============================================================================

mod address {
    use super::*;
    use fake::faker::address::raw::*;

    for_all_locales!(gen_l10d_string_test!(BuildingNumber,));
    for_all_locales!(gen_l10d_string_test!(CityName,));
    for_all_locales!(gen_l10d_string_test!(CityPrefix,));
    for_all_locales!(gen_l10d_string_test!(CitySuffix,));
    for_all_locales!(gen_l10d_string_test!(CountryCode,));
    for_all_locales!(gen_l10d_string_test!(CountryName,));
    for_all_locales!(gen_l10d_string_test_with_arg!(Geohash, 12,));
    for_all_locales!(gen_l10d_string_test!(Latitude,));
    for_all_locales!(gen_l10d_string_test!(Longitude,));
    for_all_locales!(gen_l10d_string_test!(PostCode,));
    for_all_locales!(gen_l10d_string_test!(SecondaryAddress,));
    for_all_locales!(gen_l10d_string_test!(SecondaryAddressType,));
    for_all_locales!(gen_l10d_string_test!(StateAbbr,));
    for_all_locales!(gen_l10d_string_test!(StateName,));
    for_all_locales!(gen_l10d_string_test!(StreetName,));
    for_all_locales!(gen_l10d_string_test!(StreetSuffix,));
    for_all_locales!(gen_l10d_string_test!(TimeZone,));
    for_all_locales!(gen_l10d_string_test!(ZipCode,));
}

// ============================================================================
// Administrative (FR_FR only)
// ============================================================================

mod administrative {
    use super::*;
    use fake::faker::administrative::raw::*;

    determinism_test!(
        fake_health_insurance_code_fr,
        String,
        HealthInsuranceCode(FR_FR)
    );
}

// ============================================================================
// Automotive (specific locales only)
// ============================================================================

mod automotive {
    use super::*;
    use fake::faker::automotive::raw::*;

    determinism_test!(fake_license_plate_fr, String, LicencePlate(FR_FR));
    determinism_test!(fake_license_plate_it, String, LicencePlate(IT_IT));
}

// ============================================================================
// Barcode (EN only)
// ============================================================================

mod barcode {
    use super::*;
    use fake::faker::barcode::raw::*;

    determinism_test!(fake_isbn_en, String, Isbn(EN));
    determinism_test!(fake_isbn10_en, String, Isbn10(EN));
    determinism_test!(fake_isbn13_en, String, Isbn13(EN));
}

// ============================================================================
// Company
// ============================================================================

mod company {
    use super::*;
    use fake::faker::company::raw::*;

    for_all_locales!(gen_l10d_string_test!(Bs,));
    for_all_locales!(gen_l10d_string_test!(BsAdj,));
    for_all_locales!(gen_l10d_string_test!(BsNoun,));
    for_all_locales!(gen_l10d_string_test!(BsVerb,));
    for_all_locales!(gen_l10d_string_test!(Buzzword,));
    for_all_locales!(gen_l10d_string_test!(BuzzwordMiddle,));
    for_all_locales!(gen_l10d_string_test!(BuzzwordTail,));
    for_all_locales!(gen_l10d_string_test!(CatchPhrase,));
    for_all_locales!(gen_l10d_string_test!(CompanyName,));
    for_all_locales!(gen_l10d_string_test!(CompanySuffix,));
    for_all_locales!(gen_l10d_string_test!(Industry,));
    for_all_locales!(gen_l10d_string_test!(Profession,));
}

// ============================================================================
// Credit Card (locale-independent)
// ============================================================================

mod creditcard {
    use super::*;
    use fake::faker::creditcard::raw::*;

    determinism_test!(fake_credit_card_number_en, String, CreditCardNumber(EN));
}

// ============================================================================
// Currency
// ============================================================================

mod currency {
    use super::*;
    use fake::faker::currency::raw::*;

    for_all_locales!(gen_l10d_string_test!(CurrencyCode,));
    for_all_locales!(gen_l10d_string_test!(CurrencyName,));
    for_all_locales!(gen_l10d_string_test!(CurrencySymbol,));
}

// ============================================================================
// Filesystem (locale-independent)
// ============================================================================

mod filesystem {
    use super::*;
    use fake::faker::filesystem::raw::*;

    determinism_test!(fake_dir_path_en, String, DirPath(EN));
    determinism_test!(fake_file_extension_en, String, FileExtension(EN));
    determinism_test!(fake_file_name_en, String, FileName(EN));
    determinism_test!(fake_file_path_en, String, FilePath(EN));
    determinism_test!(fake_mime_type_en, String, MimeType(EN));
    determinism_test!(fake_semver_en, String, Semver(EN));
}

// ============================================================================
// Finance
// ============================================================================

mod finance {
    use super::*;
    use fake::faker::finance::raw::*;

    for_all_locales!(gen_l10d_string_test!(Bic,));
    for_all_locales!(gen_l10d_string_test!(Isin,));
}

// ============================================================================
// Internet
// ============================================================================

mod internet {
    use super::*;
    use fake::faker::internet::raw::*;

    for_all_locales!(gen_l10d_string_test!(DomainSuffix,));
    for_all_locales!(gen_l10d_string_test!(FreeEmailProvider,));
    for_all_locales!(gen_l10d_string_test!(FreeEmail,));
    for_all_locales!(gen_l10d_test!(IP, std::net::IpAddr,));
    for_all_locales!(gen_l10d_string_test!(IPv4,));
    for_all_locales!(gen_l10d_string_test!(IPv6,));
    for_all_locales!(gen_l10d_string_test!(MACAddress,));
    for_all_locales!(gen_l10d_string_test_with_arg!(Password, 6..12,));
    for_all_locales!(gen_l10d_string_test!(SafeEmail,));
    for_all_locales!(gen_l10d_string_test!(UserAgent,));
    for_all_locales!(gen_l10d_string_test!(Username,));
}

// ============================================================================
// Job
// ============================================================================

mod job {
    use super::*;
    use fake::faker::job::raw::*;

    for_all_locales!(gen_l10d_string_test!(Field,));
    for_all_locales!(gen_l10d_string_test!(Position,));
    for_all_locales!(gen_l10d_string_test!(Seniority,));
    for_all_locales!(gen_l10d_string_test!(Title,));
}

// ============================================================================
// Lorem
// ============================================================================

mod lorem {
    use super::*;
    use fake::faker::lorem::raw::*;

    for_all_locales!(gen_l10d_string_test_with_arg!(Paragraph, 1..3,));
    for_all_locales!(gen_l10d_test_with_arg!(Paragraphs, Vec<String>, 1..3,));
    for_all_locales!(gen_l10d_string_test_with_arg!(Sentence, (1..3),));
    for_all_locales!(gen_l10d_test_with_arg!(Sentences, Vec<String>, (1..3),));
    for_all_locales!(gen_l10d_string_test!(Word,));
    for_all_locales!(gen_l10d_test_with_arg!(Words, Vec<String>, (1..3),));
}

// ============================================================================
// Name
// ============================================================================

mod name {
    use super::*;
    use fake::faker::name::raw::*;

    for_all_locales!(gen_l10d_string_test!(Name,));
    for_all_locales!(gen_l10d_string_test!(FirstName,));
    for_all_locales!(gen_l10d_string_test!(LastName,));
    for_all_locales!(gen_l10d_string_test!(NameWithTitle,));
    for_all_locales!(gen_l10d_string_test!(Title,));
    for_all_locales!(gen_l10d_string_test!(Suffix,));
}

// ============================================================================
// Number
// ============================================================================

mod number {
    use super::*;
    use fake::faker::number::raw::*;

    for_all_locales!(gen_l10d_string_test!(Digit,));
    for_all_locales!(gen_l10d_string_test_with_arg!(NumberWithFormat, "{}",));
}

// ============================================================================
// Phone Number
// ============================================================================

mod phone_number {
    use super::*;
    use fake::faker::phone_number::raw::*;

    for_all_locales!(gen_l10d_string_test!(CellNumber,));
    for_all_locales!(gen_l10d_string_test!(PhoneNumber,));
}

// ============================================================================
// Feature-gated tests
// ============================================================================

// Color
#[cfg(feature = "random_color")]
mod color {
    use super::*;
    use fake::faker::color::raw::*;

    determinism_test!(fake_hexcolor_en, String, HexColor(EN));
    determinism_test!(fake_rgbcolor_en, String, RgbColor(EN));
    determinism_test!(fake_rgbacolor_en, String, RgbaColor(EN));
    determinism_test!(fake_hslcolor_en, String, HslColor(EN));
    determinism_test!(fake_hslacolor_en, String, HslaColor(EN));
    determinism_test!(fake_color_en, String, Color(EN));
}

// Chrono
#[cfg(feature = "chrono")]
mod chrono_tests {
    use super::*;
    use fake::{chrono::Precision, faker::chrono::raw::*};

    fn lo() -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp(53469346924, 124241).expect("datetime")
    }

    fn hi() -> chrono::DateTime<chrono::Utc> {
        lo() + chrono::Duration::days(365)
    }

    macro_rules! gen_chrono_datetime_after_test {
        ($locale:ident) => {
            paste::paste! {
                determinism_test!([<fake_datetimeafter_ $locale:lower>], String, DateTimeAfter($locale, lo()));
            }
        };
    }

    macro_rules! gen_chrono_datetime_before_test {
        ($locale:ident) => {
            paste::paste! {
                determinism_test!([<fake_datetimebefore_ $locale:lower>], String, DateTimeBefore($locale, hi()));
            }
        };
    }

    macro_rules! gen_chrono_datetime_between_test {
        ($locale:ident) => {
            paste::paste! {
                determinism_test!([<fake_datetimebetween_ $locale:lower>], String, DateTimeBetween($locale, lo(), hi()));
            }
        };
    }

    macro_rules! gen_chrono_duration_test {
        ($locale:ident) => {
            paste::paste! {
                determinism_test!([<fake_duration_ $locale:lower>], ::chrono::Duration, Duration($locale));
            }
        };
    }

    for_all_locales!(gen_l10d_string_test!(Date,));
    for_all_locales!(gen_l10d_string_test!(DateTime,));
    for_all_locales!(gen_chrono_datetime_after_test!());
    for_all_locales!(gen_chrono_datetime_before_test!());
    for_all_locales!(gen_chrono_datetime_between_test!());
    for_all_locales!(gen_chrono_duration_test!());
    for_all_locales!(gen_l10d_string_test!(Time,));

    determinism_test!(
        fake_naive_time_precision,
        ::chrono::NaiveTime,
        Precision::<6>
    );
    determinism_test!(
        fake_naive_date_time_precision,
        ::chrono::NaiveDateTime,
        Precision::<6>
    );
    determinism_test!(
        fake_date_time_precision,
        ::chrono::DateTime<::chrono::Utc>,
        Precision::<6>
    );
}

// Time
#[cfg(feature = "time")]
mod time_tests {
    use super::*;
    use fake::faker::time::raw::*;

    fn lo() -> time::OffsetDateTime {
        time::OffsetDateTime::from_unix_timestamp(53469346924).unwrap()
    }

    fn hi() -> time::OffsetDateTime {
        lo() + time::Duration::days(365)
    }

    macro_rules! gen_time_datetime_after_test {
        ($locale:ident) => {
            paste::paste! {
                determinism_test!([<fake_datetimeafter_ $locale:lower>], String, DateTimeAfter($locale, lo()));
            }
        };
    }

    macro_rules! gen_time_datetime_before_test {
        ($locale:ident) => {
            paste::paste! {
                determinism_test!([<fake_datetimebefore_ $locale:lower>], String, DateTimeBefore($locale, hi()));
            }
        };
    }

    macro_rules! gen_time_datetime_between_test {
        ($locale:ident) => {
            paste::paste! {
                determinism_test!([<fake_datetimebetween_ $locale:lower>], String, DateTimeBetween($locale, lo(), hi()));
            }
        };
    }

    macro_rules! gen_time_duration_test {
        ($locale:ident) => {
            paste::paste! {
                determinism_test!([<fake_duration_ $locale:lower>], ::time::Duration, Duration($locale));
            }
        };
    }

    for_all_locales!(gen_l10d_string_test!(Date,));
    for_all_locales!(gen_l10d_string_test!(DateTime,));
    for_all_locales!(gen_time_datetime_after_test!());
    for_all_locales!(gen_time_datetime_before_test!());
    for_all_locales!(gen_time_datetime_between_test!());
    for_all_locales!(gen_time_duration_test!());
    for_all_locales!(gen_l10d_string_test!(Time,));
}

// HTTP
#[cfg(feature = "http")]
mod http_tests {
    use super::*;
    use fake::faker::http::raw::*;

    determinism_test!(fake_rfc_status_code_en, String, RfcStatusCode(EN));
    determinism_test!(fake_valid_status_code_en, String, ValidStatusCode(EN));
}

// ULID
#[cfg(feature = "ulid")]
mod ulid_tests {
    use super::*;
    determinism_test!(fake_ulid, ulid::Ulid, Faker);
}

// Ferroid
#[cfg(feature = "ferroid")]
mod ferroid_tests {
    use super::*;
    use fake::ferroid::*;

    determinism_test!(fake_ferroid_ulid_f, ferroid::id::ULID, Faker);
    determinism_test!(fake_ferroid_ulid, ferroid::id::ULID, FerroidULID);
    determinism_test!(
        fake_ferroid_snowflake_twitter_f,
        ferroid::id::SnowflakeTwitterId,
        Faker
    );
    determinism_test!(
        fake_ferroid_snowflake_twitter,
        ferroid::id::SnowflakeTwitterId,
        FerroidTwitterId
    );
    determinism_test!(
        fake_ferroid_snowflake_mastodon_f,
        ferroid::id::SnowflakeMastodonId,
        Faker
    );
    determinism_test!(
        fake_ferroid_snowflake_mastodon,
        ferroid::id::SnowflakeMastodonId,
        FerroidMastodonId
    );
    determinism_test!(
        fake_ferroid_snowflake_discord_f,
        ferroid::id::SnowflakeDiscordId,
        Faker
    );
    determinism_test!(
        fake_ferroid_snowflake_discord,
        ferroid::id::SnowflakeDiscordId,
        FerroidDiscordId
    );
    determinism_test!(
        fake_ferroid_snowflake_instagram_f,
        ferroid::id::SnowflakeInstagramId,
        Faker
    );
    determinism_test!(
        fake_ferroid_snowflake_instagram,
        ferroid::id::SnowflakeInstagramId,
        FerroidInstagramId
    );
}

// UUID
#[cfg(feature = "uuid")]
mod uuid_tests {
    use super::*;
    use fake::uuid::*;

    determinism_test!(fake_uuid_v1, uuid::Uuid, UUIDv1);
    determinism_test!(fake_uuid_v3, uuid::Uuid, UUIDv3);
    determinism_test!(fake_uuid_v4, uuid::Uuid, UUIDv4);
    determinism_test!(fake_uuid_v5, uuid::Uuid, UUIDv5);
}

// Geo
#[cfg(feature = "geo-types")]
mod geo {
    use super::*;

    determinism_test!(fake_geo_coord_f64, geo_types::Coord<f64>, Faker);
    determinism_test!(fake_geo_coord_u64, geo_types::Coord<u64>, Faker);
    determinism_test!(fake_geo_line_f64, geo_types::Line<f64>, Faker);
    determinism_test!(fake_geo_line_u64, geo_types::Line<u64>, Faker);
    determinism_test!(fake_geo_linestring_f64, geo_types::LineString<f64>, Faker);
    determinism_test!(fake_geo_linestring_u64, geo_types::LineString<u64>, Faker);
    determinism_test!(
        fake_geo_multilinestring_f64,
        geo_types::MultiLineString<f64>,
        Faker
    );
    determinism_test!(
        fake_geo_multilinestring_u64,
        geo_types::MultiLineString<u64>,
        Faker
    );
    determinism_test!(fake_geo_point_f64, geo_types::Point<f64>, Faker);
    determinism_test!(fake_geo_point_u64, geo_types::Point<u64>, Faker);
    determinism_test!(fake_geo_multipoint_f64, geo_types::MultiPoint<f64>, Faker);
    determinism_test!(fake_geo_multipoint_u64, geo_types::MultiPoint<u64>, Faker);
    determinism_test!(
        fake_geo_multipolygon_f64,
        geo_types::MultiPolygon<f64>,
        Faker
    );
    determinism_test!(
        fake_geo_multipolygon_u64,
        geo_types::MultiPolygon<u64>,
        Faker
    );
    determinism_test!(fake_geo_polygon_f64, geo_types::Polygon<f64>, Faker);
    determinism_test!(fake_geo_polygon_u64, geo_types::Polygon<u64>, Faker);
    determinism_test!(fake_geo_rect_f64, geo_types::Rect<f64>, Faker);
    determinism_test!(fake_geo_rect_u64, geo_types::Rect<u64>, Faker);
    determinism_test!(fake_geo_triangle_f64, geo_types::Triangle<f64>, Faker);
    determinism_test!(fake_geo_triangle_u64, geo_types::Triangle<u64>, Faker);
    determinism_test!(fake_geo_geometry_f64, geo_types::Geometry<f64>, Faker);
    determinism_test!(fake_geo_geometry_u64, geo_types::Geometry<u64>, Faker);
    determinism_test!(
        fake_geo_geometry_collection_f64,
        geo_types::GeometryCollection<f64>,
        Faker
    );
    determinism_test!(
        fake_geo_geometry_collection_u64,
        geo_types::GeometryCollection<u64>,
        Faker
    );
}

// Decimal
#[cfg(feature = "rust_decimal")]
mod decimal {
    use super::*;
    use fake::decimal::*;
    use rust_decimal as rs;

    determinism_test!(default_decimal, rs::Decimal, Decimal);
    determinism_test!(negative_decimal, rs::Decimal, NegativeDecimal);
    determinism_test!(positive_decimal, rs::Decimal, PositiveDecimal);
    determinism_test!(no_decimal_points, rs::Decimal, NoDecimalPoints);
}

// BigDecimal
#[cfg(feature = "bigdecimal")]
mod bigdecimal {
    use super::*;
    use bigdecimal_rs as bd;
    use fake::bigdecimal::*;

    determinism_test!(default_bigdecimal, bd::BigDecimal, BigDecimal);
    determinism_test!(negative_bigdecimal, bd::BigDecimal, NegativeBigDecimal);
    determinism_test!(positive_bigdecimal, bd::BigDecimal, PositiveBigDecimal);
    determinism_test!(no_bigdecimal_points, bd::BigDecimal, NoBigDecimalPoints);
}

// Serde JSON
#[cfg(feature = "serde_json")]
mod serde_json_tests {
    use super::*;
    determinism_test!(fake_serde_json, serde_json::Value, Faker);
}

// BSON ObjectId
#[cfg(feature = "bson_oid")]
mod bson_oid {
    use super::*;
    use bson::oid::ObjectId;
    determinism_test!(fake_bson_oid, ObjectId, Faker);
}

// Base64
#[cfg(feature = "base64")]
mod base64_tests {
    use super::*;
    use fake::base64::*;
    determinism_test!(fake_base64, Base64Value, Faker);
    determinism_test!(fake_url_safe_base64, UrlSafeBase64Value, Faker);
}
