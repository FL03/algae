/*
   Appellation: algae <library>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Algae
//!
//! Algae is a comprehensive collection of algorithms and data-structures written in Rust.
#[cfg(feature = "graph")]
pub use algae_graph as graph;
#[cfg(feature = "merkle")]
pub use algae_merkle as merkle;
#[cfg(feature = "mmr")]
pub use algae_mmr as mmr;
#[cfg(feature = "queue")]
pub use algae_queue as queue;

pub use algae_core as core;
pub mod list;

pub mod prelude {
    pub use crate::core::prelude::*;
    #[cfg(feature = "graph")]
    pub use algae_graph::graph;
    #[cfg(feature = "merkle")]
    pub use algae_merkle::prelude::*;
    #[cfg(feature = "mmr")]
    pub use algae_mmr::*;
    #[cfg(feature = "queue")]
    pub use algae_queue::prelude::*;

    pub use crate::list::*;
}
