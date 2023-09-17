/*
    Appellation: nodes <merkle>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{combine_hash_str, merkle_hash, Payload};
use decanter::prelude::{Hashable, H256};
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Node<T = String>
where
    T: Default + ToString,
{
    pub data: Payload<T>,
    pub hash: H256,
}

impl<T> Node<T>
where
    T: Default + ToString,
{
    pub fn new(data: Payload<T>, hash: H256) -> Self {
        Self { data, hash }
    }
}

impl<T> Hashable for Node<T>
where
    T: Default + ToString,
{
    fn hash(&self) -> H256 {
        merkle_hash(self.data.to_string())
    }
}

impl<T> From<(Node<T>, Node<T>)> for Node<T>
where
    T: Default + ToString,
{
    fn from(data: (Node<T>, Node<T>)) -> Self {
        let hash = merkle_hash(combine_hash_str(&data.0.hash, &data.1.hash));
        let data = Payload::Node(Box::new(data.0), Box::new(data.1));
        Self::new(data, hash)
    }
}

impl<T> std::fmt::Display for Node<T>
where
    T: Default + ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = serde_json::json!({
            "data": self.data.to_string(),
            "hash": self.hash,
        });
        write!(f, "{}", msg)
    }
}
