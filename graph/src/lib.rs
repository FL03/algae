/*
   Appellation: algae-graph <library>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description:
*/
#[doc(inline)]
pub use self::{errors::*, graphable::*, primitives::*, utils::*};

pub(crate) mod errors;
pub(crate) mod graphable;
pub(crate) mod primitives;
pub(crate) mod utils;

pub mod directed;
pub mod undirected;
