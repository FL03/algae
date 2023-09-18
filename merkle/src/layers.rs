/*
    Appellation: layers <merkle>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Node;
use decanter::prelude::Hashable;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

// pub fn build_new_merkle_layer<T: ToString>(left: MerkleNode<T>, right: MerkleNode)
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Layer<T = String>(Vec<Node<T>>)
where
    T: Hashable;

impl<T> Layer<T>
where
    T: Hashable,
{
    pub fn new(data: impl IntoIterator<Item = Node<T>>) -> Self {
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

impl<T> From<Vec<Node<T>>> for Layer<T>
where
    T: Hashable,
{
    fn from(data: Vec<Node<T>>) -> Self {
        Self::new(data)
    }
}

impl<T> From<(Node<T>, Node<T>)> for Layer<T>
where
    T: Hashable,
{
    fn from(data: (Node<T>, Node<T>)) -> Self {
        Self::new(vec![data.0, data.1])
    }
}
