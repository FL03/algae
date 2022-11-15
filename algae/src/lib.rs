/*
   Appellation: algae <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Algae is a comprehensive collection of algorithms and data-structures
*/
#[cfg(feature = "graph")]
pub use algae_graph as graph;
#[cfg(feature = "merkle")]
pub use algae_merkle as merkle;
#[cfg(feature = "mmr")]
pub use algae_mmr as mmr;

pub mod trees;

pub mod prelude {
    #[cfg(feature = "graph")]
    pub use super::graph::*;
    #[cfg(feature = "merkle")]
    pub use super::merkle::*;
    #[cfg(feature = "mmr")]
    pub use super::mmr::*;
}
