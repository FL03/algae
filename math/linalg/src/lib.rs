/*
   Appellation: algae-linalg <lib>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # algae-linalg

pub use self::{primitives::*, specs::*, utils::*};

pub mod vs;

// pub(crate) use concision_core as core;

pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;

pub mod prelude {
    pub use crate::vs::*;
    pub use crate::primitives::*;
    pub use crate::specs::*;
    pub use crate::utils::*;
}
