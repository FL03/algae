/*
   Appellation: proof <merkle>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use super::path::proof_path;
use crate::{MerkleDimension, MerkleTree};
use decanter::prelude::H256;

pub struct Prover {
    cursor: usize,
    proof: Vec<H256>,
    tree: MerkleTree,
}

impl Prover {
    pub fn new(tree: MerkleTree) -> Self {
        Self {
            cursor: 0,
            proof: Vec::new(),
            tree,
        }
    }
    /// Returns the proof for the given index
    pub fn prove(&mut self, index: usize) -> Vec<H256> {
        self.proof = self.tree.proof(index);
        self.proof.clone()
    }
}

impl Iterator for Prover {
    type Item = Vec<H256>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.proof.len() {
            let item = self.prove(self.cursor);
            self.cursor += 1;
            Some(item)
        } else {
            None
        }
    }
}

// Returns the proof for the given index
pub fn merkle_proof(dim: MerkleDimension, nodes: Vec<H256>, index: usize) -> Vec<H256> {
    let mut proof: Vec<H256> = Vec::new();
    let mut offset: usize = 0;
    let mut leaf_size = dim.leafs;

    let proof_index = proof_path(index, leaf_size);

    for i in 0..dim.depth - 1 {
        proof.push(nodes[offset + proof_index[i]]);
        if leaf_size % 2 != 0 {
            leaf_size += 1;
        }
        offset += leaf_size;
        leaf_size /= 2;
    }
    proof
}
