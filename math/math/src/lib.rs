/*
   Appellation: algae-math <lib>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # algae-math

pub use self::{factorials::*, primitives::*, specs::*, utils::*};

pub mod calculus;
pub mod num;

pub use algae_linalg as linalg;

pub use algae_statistics as stats;

pub(crate) mod factorials;

// pub(crate) use concision_core as core;

pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;

pub mod prelude {
    pub use crate::calculus::*;
    pub use crate::num::*;
    pub use crate::primitives::*;
    pub use crate::specs::*;
    pub use crate::utils::*;
}
