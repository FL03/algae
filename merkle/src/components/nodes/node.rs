/*
    Appellation: nodes <merkle>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{combine, merkle_hash, Payload};
use scsys::prelude::{H256, Hashable};
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Node<T: ToString> {
    pub data: Payload<T>,
    pub hash: H256,
}

impl<T: ToString> Node<T> {
    pub fn new(data: Payload<T>, hash: H256) -> Self {
        Self { data, hash }
    }
}

impl<T: Serialize + ToString> Hashable for Node<T> {
    fn hash(&self) -> H256 {
        merkle_hash(&self.data)
    }
}

impl<T: ToString> std::convert::From<(Node<T>, Node<T>)> for Node<T> {
    fn from(data: (Node<T>, Node<T>)) -> Self {
        let hash = merkle_hash(combine(&data.0.hash, &data.1.hash));
        let data = Payload::Node(Box::new(data.0), Box::new(data.1));
        Self::new(data, hash)
    }
}

impl<T: Clone + Serialize + ToString> std::convert::From<T> for Node<T> {
    fn from(data: T) -> Self {
        Self::new(Payload::from(data.clone()), merkle_hash(data))
    }
}
