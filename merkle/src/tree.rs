/*
    Appellation: tree <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{create_merkle_tree, MerkleDimension, MerkleShape, MerkleTreeSpec};
use decanter::prelude::{Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MerkleTree {
    pub dim: MerkleDimension,
    pub nodes: Vec<H256>,
}

impl MerkleTree {
    pub fn new(dim: MerkleDimension, nodes: Vec<H256>) -> Self {
        Self { dim, nodes }
    }
}

impl MerkleTreeSpec for MerkleTree {
    fn dim(&self) -> MerkleDimension {
        self.dim.clone()
    }
    fn nodes(&self) -> Vec<H256> {
        self.nodes.clone()
    }
}

impl std::fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.nodes)
    }
}

impl<T: Hashable> From<&[T]> for MerkleTree {
    fn from(data: &[T]) -> Self {
        let (dim, nodes) = create_merkle_tree::<T>(data);
        Self::new(MerkleDimension::from(dim), nodes)
    }
}

impl From<(Box<dyn MerkleShape>, Vec<H256>)> for MerkleTree {
    fn from(data: (Box<dyn MerkleShape>, Vec<H256>)) -> Self {
        Self::new(MerkleDimension::from(data.0), data.1)
    }
}

impl From<Vec<H256>> for MerkleTree {
    fn from(data: Vec<H256>) -> Self {
        Self::from(create_merkle_tree(&data))
    }
}
