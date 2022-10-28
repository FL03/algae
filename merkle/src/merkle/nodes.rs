/*
    Appellation: nodes <merkle>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{combine, leaves::Leaf, merkle_hash};
use scsys::crypto::hashes::H256;
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum MerklePayload<T: ToString> {
    Leaf(Leaf<T>),
    Node(Box<MerkleNode<T>>, Box<MerkleNode<T>>),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MerkleNode<T: ToString> {
    pub data: MerklePayload<T>,
    pub hash: H256,
}

impl<T: ToString> MerkleNode<T> {
    pub fn new(data: MerklePayload<T>, hash: H256) -> Self {
        Self { data, hash }
    }
}

impl<T: ToString> std::convert::From<(MerkleNode<T>, MerkleNode<T>)> for MerkleNode<T> {
    fn from(data: (MerkleNode<T>, MerkleNode<T>)) -> Self {
        let concat = combine(&data.0.hash, &data.1.hash);
        let hash = merkle_hash(concat);
        let data = MerklePayload::Node(Box::new(data.0), Box::new(data.1));
        Self::new(data, H256::from(hash))
    }
}

impl<T: ToString> std::convert::From<T> for MerkleNode<T> {
    fn from(data: T) -> Self {
        let hash = merkle_hash(data.to_string());
        let data = MerklePayload::Leaf(Leaf::new(data));
        Self::new(data, hash)
    }
}
