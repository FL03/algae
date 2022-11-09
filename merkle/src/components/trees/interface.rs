/*
    Appellation: tree <merkle>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Merkle Tree def...
*/
use crate::components::{layers::Layer, nodes::Node};
use scsys::prelude::{Hashable, H256};
use serde::{Deserialize, Serialize};
use std::string::ToString;

/// Implements a complete merkle tree
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Tree<T: ToString> {
    pub leaves: Vec<T>,
    pub root: Node<T>,
}

impl<T: ToString> Tree<T> {
    pub fn new(leaves: Vec<T>, root: Node<T>) -> Self {
        Self { leaves, root }
    }
    pub fn root_hash(&self) -> H256 {
        self.root.hash()
    }
}

impl<II: IntoIterator> std::convert::From<II> for Tree<II::Item>
where
    <II as IntoIterator>::Item: Clone + ToString,
{
    fn from(data: II) -> Self {
        let leaves = data.into_iter().collect::<Vec<_>>();

        let mut layer: Vec<_> = leaves.iter().cloned().map(Node::from).collect();

        while layer.len() != 1 {
            layer = Layer::new(layer).into();
        }

        match layer.pop() {
            Some(root) => Self::new(leaves, root),
            None => panic!("You should not have built an empty tree"),
        }
    }
}
