/*
   Appellation: algae <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Algae is a comprehensive collection of algorithms and data-structures
*/
#[cfg(feature = "core")]
pub use algae_core as core;
#[cfg(feature = "derive")]
pub use algae_derive::*;
#[cfg(feature = "macros")]
pub use algae_macros::*;
