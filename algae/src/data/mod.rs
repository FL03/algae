/*
   Appellation: data <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
*/
#[doc(inline)]
pub use utils::*;

#[doc(inline)]
#[cfg(feature = "trees")]
pub use algae_trees as trees;

pub mod proofs;

mod utils {}
