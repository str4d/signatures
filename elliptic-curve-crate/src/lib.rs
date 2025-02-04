//! General purpose Elliptic Curve Cryptography (ECC) support, including types
//! and traits for representing various elliptic curve forms, scalars, points,
//! and public/secret keys composed thereof.
//!
//! ## Minimum Supported Rust Version
//!
//! Rust **1.34** or higher.
//!
//! Minimum supported Rust version can be changed in the future, but it will be
//! done with a minor version bump.

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/meta/master/logo_small.png",
    html_root_url = "https://docs.rs/elliptic-curve/0.1.0"
)]

#[cfg(feature = "std")]
extern crate std;

pub use generic_array;

pub mod error;

// TODO(tarcieri): other curve forms
#[cfg(feature = "weierstrass")]
pub mod weierstrass;

pub use self::error::Error;
