/*
    Appellation: algae-mmr <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{builder::*, mmr::*, primitives::*, utils::*};

mod builder;
mod mmr;
mod primitives;
mod utils;

pub mod cmp;
pub mod proofs;
pub mod stores;

pub mod prelude {
    pub use super::mmr::MerkleMountainRange;
}