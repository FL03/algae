/*
   Appellation: algae <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Algae is a comprehensive collection of algorithms and data-structures
*/
#[cfg(feature = "core")]
pub use algae_core::*;
#[cfg(feature = "derive")]
pub use algae_derive::*;
#[cfg(feature = "macros")]
pub use algae_macros::*;
#[cfg(feature = "merkle")]
pub use algae_merkle as merkle;

pub mod prelude {
    #[cfg(feature = "core")]
    pub use super::{
        search, sort,
        structs::{BTree, BTreeProps},
        CoinChange, EditDistance,
    };

    #[cfg(feature = "merkle")]
    pub use super::merkle::{crypto::*, merkle::*, proofs::*};
}
