/*
   Appellation: algae <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Algae is a comprehensive collection of algorithms and data-structures
*/
#[doc(inline)]
#[cfg(feature = "default")]
pub use crate::{actors::*, core::*, data::*};

mod actors;
mod core;
mod data;
