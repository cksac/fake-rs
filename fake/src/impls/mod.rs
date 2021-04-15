//! This module contains implementations of `Dummy` trait

#[cfg(feature = "chrono")]
pub mod chrono;
#[cfg(feature = "http")]
pub mod http;
pub mod std;
#[cfg(feature = "semver")]
pub mod semver;
