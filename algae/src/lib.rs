/*
   Appellation: algae <library>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Algae
//!
//! Algae is a comprehensive collection of algorithms and data-structures written in Rust.
#[cfg(feature = "graph")]
pub use algae_graph as graph;
#[cfg(feature = "math")]
pub use algae_math as math;
#[cfg(feature = "merkle")]
pub use algae_merkle as merkle;
#[cfg(feature = "mmr")]
pub use algae_mmr as mmr;

pub mod list;

pub mod prelude {
    #[cfg(feature = "graph")]
    pub use super::graph;
    // pub use super::graph::*;
    #[cfg(feature = "math")]
    pub use algae_math::prelude::*;
    #[cfg(feature = "merkle")]
    pub use super::merkle::prelude::*;
    #[cfg(feature = "mmr")]
    pub use super::mmr::*;

    pub use super::list::*;

}
