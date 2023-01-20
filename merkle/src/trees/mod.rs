/*
   Appellation: interfaces <merkle>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{interface::*, tree::*, utils::*};

pub(crate) mod interface;
pub(crate) mod tree;

use crate::{proofs::merkle_proof, MerkleDimension, MerkleShape};
use decanter::prelude::{Hashable, H256};

pub trait MerkleTreeWrapper {
    fn new(dim: MerkleDimension, nodes: Vec<H256>) -> Self;
    fn create<T>(data: &[T]) -> Self
    where
        Self: Sized,
        T: Hashable,
    {
        let (dim, nodes) = create_merkle_tree(data);

        Self::new(dim.into(), nodes)
    }
    fn dim(&self) -> MerkleDimension;
    fn nodes(&self) -> Vec<H256>;
    // Returns the proof for the given index
    fn proof(&self, index: usize) -> Vec<H256> {
        merkle_proof(self.dim(), self.nodes(), index)
    }
    // Returns the root hash of the merkle tree
    fn root(&self) -> H256 {
        self.nodes()[self.dim().size() - 1]
    }
}

pub trait MerkleTreeWrapperExt: MerkleTreeWrapper {
    // Writes the injected nodes to the console for viewing purposes
    fn print(&self) -> &Self {
        for i in 0..self.dim().size {
            println!("{:?}", self.nodes()[i]);
        }
        self
    }
}
pub(crate) mod utils {
    use crate::{add_hash, MerkleDimension, MerkleShape};
    use decanter::prelude::{Hashable, H256};

    pub fn create_merkle_tree<T>(data: &[T]) -> (Box<dyn MerkleShape>, Vec<H256>)
    where
        T: Hashable,
    {
        // unimplemented!()
        let mut length = data.len();
        let mut nodes = Vec::new();
        let mut last_level = Vec::new();
        for i in data {
            let h: H256 = i.hash();
            last_level.push(h);
            nodes.push(h);
        }
        let mut depth = 1;
        while length > 1 {
            if length % 2 != 0 {
                last_level.push(data[length - 1].hash());
                nodes.push(data[length - 1].hash());
                length += 1;
            }
            let mut temp = Vec::new();
            for i in 0..length / 2 {
                let h: H256 = add_hash(&last_level[2 * i], &last_level[2 * i + 1]);
                temp.push(h);
                nodes.push(h);
            }
            last_level = temp.clone();
            length /= 2;
            depth += 1;
        }
        let dim = MerkleDimension::new(depth, data.len(), nodes.len());
        (Box::new(dim), nodes)
    }
}
