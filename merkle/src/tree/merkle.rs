use std::ops::{Index, IndexMut, Range};

/*
    Appellation: merkle <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::proofs::merkle_proof;
use crate::{create_merkle_tree, MerkleDimension};
use decanter::prelude::{Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MerkleTree {
    dim: MerkleDimension,
    nodes: Vec<H256>,
}

impl MerkleTree {
    pub fn new(dim: MerkleDimension) -> Self {
        Self {
            dim,
            nodes: Vec::new(),
        }
    }
    pub fn new_with_nodes(dim: MerkleDimension, nodes: Vec<H256>) -> Self {
        Self { dim, nodes }
    }
    pub fn dim(&self) -> MerkleDimension {
        self.dim
    }
    pub fn proof(&self, index: usize) -> Vec<H256> {
        merkle_proof(self.dim(), self.nodes().clone(), index)
    }
    pub fn root(&self) -> H256 {
        self[self.dim().size() - 1]
    }
    pub fn nodes(&self) -> &Vec<H256> {
        &self.nodes
    }
    pub fn nodes_mut(&mut self) -> &mut Vec<H256> {
        &mut self.nodes
    }

    
}

impl std::fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.nodes)
    }
}

impl<T> From<&[T]> for MerkleTree
where
    T: Hashable,
{
    fn from(data: &[T]) -> Self {
        let (dim, nodes) = create_merkle_tree::<T>(data);
        Self {
            dim: MerkleDimension::from(dim),
            nodes,
        }
    }
}

impl Index<usize> for MerkleTree {
    type Output = H256;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl IndexMut<usize> for MerkleTree {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl Index<Range<usize>> for MerkleTree {
    type Output = [H256];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.nodes[index]
    }
}

impl IndexMut<Range<usize>> for MerkleTree {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
