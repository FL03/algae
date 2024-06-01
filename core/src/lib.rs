/*
   Appellation: algae-core <library>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Algae Core
//!
//! Algae is a comprehensive collection of algorithms and data-structures written in Rust.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{error::{Error, Result}, traits::prelude::*};

pub mod error;
pub mod traits;

pub mod prelude {
    pub use crate::traits::prelude::*;
}
