/*
    Appellation: tree <merkle>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Merkle Tree def...
*/
use super::{layers::MerkleLayer, nodes::MerkleNode};
use scsys::crypto::hashes::H256;
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MerkleTree<T: ToString> {
    pub leaves: Vec<T>,
    pub root: MerkleNode<T>,
}

impl<T: ToString> MerkleTree<T> {
    pub fn new(leaves: Vec<T>, root: MerkleNode<T>) -> Self {
        Self { leaves, root }
    }
    pub fn root_hash(&self) -> H256 {
        self.root.hash.clone()
    }
}

/// Implements a standard method of generating new Merkle Trees from leaves
impl<II: IntoIterator> std::convert::From<II> for MerkleTree<II::Item>
where
    <II as IntoIterator>::Item: Clone + ToString,
{
    fn from(data: II) -> Self {
        let leaves = data.into_iter().collect::<Vec<_>>();

        let mut layer: Vec<_> = leaves.iter().cloned().map(MerkleNode::from).collect();

        while layer.len() != 1 {
            layer = MerkleLayer::new(layer).into();
        }

        match layer.pop() {
            Some(root) => MerkleTree::new(leaves, root),
            None => panic!("You should not have built an empty tree"),
        }
    }
}
