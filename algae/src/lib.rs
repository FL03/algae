/*
   Appellation: algae <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Algae is intended to be a collection of optimized algorithms
*/
#[doc(inline)]
#[cfg(feature = "default")]
pub use crate::{actors::*, core::*, data::*};

mod actors;
mod core;
mod data;