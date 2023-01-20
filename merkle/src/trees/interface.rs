/*
    Appellation: tree <merkle>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Merkle Tree def...
*/
use crate::{Layer, Node};
use decanter::prelude::{Hashable, H256};
use serde::{Deserialize, Serialize};
use std::string::ToString;

/// Implements a complete merkle tree
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Tree<T: Serialize + ToString> {
    pub leaves: Vec<T>,
    pub root: Node<T>,
}

impl<T: Serialize + ToString> Tree<T> {
    pub fn new(leaves: Vec<T>, root: Node<T>) -> Self {
        Self { leaves, root }
    }
    pub fn root_hash(&self) -> H256 {
        self.root.hash()
    }
}

impl<II: IntoIterator> std::convert::From<II> for Tree<II::Item>
where
    <II as IntoIterator>::Item: Clone + Serialize + ToString,
{
    fn from(data: II) -> Self {
        let leaves = data.into_iter().collect::<Vec<_>>();

        let mut layer: Vec<_> = leaves.iter().cloned().map(Node::from).collect();

        while layer.len() != 1 {
            layer = Layer::from(layer).layer().clone();
        }

        match layer.pop() {
            Some(root) => Self::new(leaves, root),
            None => panic!("You should not have built an empty tree"),
        }
    }
}
