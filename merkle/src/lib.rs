/*
   Appellation: algae-merkle <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
*/
#[cfg(test)]
extern crate hex_literal;
#[doc(inline)]
pub use self::{layers::*, nodes::*, payloads::*, shape::*, tree::*, utils::*};

pub(crate) mod layers;
pub(crate) mod nodes;
pub(crate) mod payloads;
pub(crate) mod shape;
pub(crate) mod tree;
pub(crate) mod utils;

pub mod proofs;

pub mod prelude {
    pub use crate::layers::*;
    pub use crate::nodes::*;
    pub use crate::payloads::*;
    pub use crate::shape::*;
    pub use crate::tree::*;
    pub use crate::utils::*;
}
