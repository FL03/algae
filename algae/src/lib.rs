/*
   Appellation: algae <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Algae is a comprehensive collection of algorithms and data-structures
*/

pub mod graphs;
pub mod trees;

#[cfg(feature = "merkle")]
pub use algae_merkle as merkle;

pub mod prelude {

    #[cfg(feature = "merkle")]
    pub use super::merkle::{components::*, proofs::*};
}
