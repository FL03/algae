/*
    Appellation: algae-spline <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Algae Spline
//!
//!
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use self::error::{Result, SplineError};
pub use self::traits::prelude::*;

pub mod bspline;
pub mod error;
pub mod traits;

pub mod prelude {
    pub use super::bspline::prelude::*;
    pub use super::error::*;
    pub use super::traits::prelude::*;
}
