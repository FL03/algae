/*
   Appellation: algae <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Algae is a comprehensive collection of algorithms and data-structures
*/
#[doc(inline)]
pub use crate::{actors::*, core::*, data::*};
#[doc(inline)]
#[cfg(feature = "derive")]
pub use algae_derive::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use algae_macros::*;

mod actors;
mod core;
mod data;
