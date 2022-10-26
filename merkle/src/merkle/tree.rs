/*
    Appellation: tree <merkle>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Merkle Tree def...
*/
use super::MerkleNode;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::string::ToString;

// pub fn build_new_merkle_layer<T: ToString>(left: MerkleNode<T>, right: MerkleNode)
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MerkleLayer<T: ToString>(Vec<MerkleNode<T>>);

impl<T: ToString> MerkleLayer<T> {
    pub fn new(data: Vec<MerkleNode<T>>) -> Self {
        let layer = data.into_iter().batching(|it| match it.next() {
            Some(l) => match it.next() {
                Some(r) => Some(MerkleNode::from((l, r))),
                None => Some(l),
            },
            None => None,
        });

        Self(layer.collect::<Vec<_>>())
    }
}
impl<T: ToString> std::convert::Into<Vec<MerkleNode<T>>> for MerkleLayer<T> {
    fn into(self) -> Vec<MerkleNode<T>> {
        self.0
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MerkleTree<T: ToString> {
    pub leaves: Vec<T>,
    pub root: MerkleNode<T>,
}

impl<T: ToString> MerkleTree<T> {
    pub fn new(leaves: Vec<T>, root: MerkleNode<T>) -> Self {
        Self { leaves, root }
    }
    pub fn root_hash(&self) -> String {
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
