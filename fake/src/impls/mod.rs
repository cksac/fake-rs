//! This module contains implementations of `Dummy` trait

#[cfg(feature = "random_color")]
pub mod color;
#[cfg(feature = "chrono")]
pub mod chrono;
#[cfg(feature = "rust_decimal")]
pub mod decimal;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "semver")]
pub mod semver;
pub mod std;
#[cfg(feature = "time")]
pub mod time;
#[cfg(feature = "uuid")]
pub mod uuid;
