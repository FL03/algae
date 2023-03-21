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

use decanter::prelude::{Hashable, H256};
use proofs::merkle_proof;

pub trait MerkleTreeSpec: Clone {
    // Returns the dimension of the given tree
    fn dim(&self) -> MerkleDimension;
    // Returns the nodes of the given tree
    fn nodes(&self) -> Vec<H256>;
    // Returns the proof for the given index
    fn proof(&self, index: usize) -> Vec<H256> {
        merkle_proof(self.dim(), self.nodes(), index)
    }
    // Writes the injected nodes to the console for viewing purposes
    fn print(&self) -> &Self {
        for i in 0..self.dim().size {
            println!("{:?}", self.nodes()[i]);
        }
        self
    }
    // Returns the root hash of the merkle tree
    fn root(&self) -> H256 {
        self.nodes()[self.dim().size() - 1]
    }
}
