/*
   Appellation: algae <library>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Algae
//!
//! Algae is a comprehensive collection of algorithms and data-structures written in Rust.
#[doc(inline)]
pub use algae_core::*;
#[cfg(feature = "graph")]
pub use algae_graph as graph;
#[cfg(feature = "merkle")]
pub use algae_merkle as merkle;
// #[cfg(feature = "mmr")]
// pub use algae_mmr as mmr;

pub mod list;

pub mod prelude {
    pub use algae_core::prelude::*;
    #[cfg(feature = "graph")]
    pub use algae_graph::graph;
    #[cfg(feature = "merkle")]
    pub use algae_merkle::prelude::*;
    // #[cfg(feature = "mmr")]
    // pub use algae_mmr::*;

    pub use crate::list::*;
}
