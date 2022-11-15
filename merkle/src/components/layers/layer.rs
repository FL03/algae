/*
    Appellation: layers <merkle>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Merkle Tree def...
*/
use crate::components::nodes::Node;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::string::ToString;

// pub fn build_new_merkle_layer<T: ToString>(left: MerkleNode<T>, right: MerkleNode)
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Layer<T: ToString>(Vec<Node<T>>);

impl<T: ToString> Layer<T> {
    pub fn new(data: Vec<Node<T>>) -> Self {
        let layer = data.into_iter().batching(|it| match it.next() {
            Some(l) => match it.next() {
                Some(r) => Some(Node::from((l, r))),
                None => Some(l),
            },
            None => None,
        });

        Self(layer.collect::<Vec<_>>())
    }
    pub fn layer(&self) -> &Vec<Node<T>> {
        &self.0
    }
}

impl<T: ToString> std::convert::From<Vec<Node<T>>> for Layer<T> {
    fn from(data: Vec<Node<T>>) -> Self {
        Self::new(data)
    }
}