//! This module contains implementations of `Dummy` trait

#[cfg(feature = "bigdecimal")]
pub mod bigdecimal;
#[cfg(feature = "chrono")]
pub mod chrono;
#[cfg(feature = "chrono-tz")]
pub mod chrono_tz;
#[cfg(feature = "random_color")]
pub mod color;
#[cfg(feature = "rust_decimal")]
pub mod decimal;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "sea-orm")]
pub mod sea_orm;
#[cfg(feature = "semver")]
pub mod semver;
#[cfg(feature = "serde_json")]
pub mod serde_json;
pub mod std;
#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "uuid")]
pub mod uuid;
#[cfg(feature = "zerocopy")]
pub mod zerocopy_byteorder;
