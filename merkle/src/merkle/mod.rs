/*
    Appellation: merkle <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This module is dedicated towards the implementation of a complete, WebAssembley native merkle tree for blockchain's and other advanced applications
*/
pub use self::{layers::*, leaves::*, nodes::*, tree::*, utils::*};

pub(crate) mod layers;
pub(crate) mod leaves;
pub(crate) mod nodes;
pub(crate) mod tree;

pub(crate) mod utils {
    use super::nodes::{MerkleNode, MerklePayload};
    use scsys::crypto::hashes::H256;
    use serde::Serialize;
    use sha2::{Digest, Sha256};
    use std::string::ToString;

    pub fn combine<T: ToString>(a: &T, b: &T) -> String {
        format!("{}{}", a.to_string(), b.to_string())
    }

    pub fn hasher<T: serde::Serialize>(data: T) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(&data).unwrap().as_bytes());
        hasher.finalize().as_slice().to_owned()
    }

    pub fn compute_hash<T: ToString>(data: T) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.to_string().as_bytes());
        // hasher.finalize().as_slice().to_owned()
        format!("{:x}", hasher.finalize())
    }

    pub fn new_leaf<T>(val: T) -> MerkleNode<T>
    where
        T: ToString,
    {
        MerkleNode {
            hash: merkle_hash(val.to_string()),
            data: MerklePayload::Leaf(val),
        }
    }

    pub fn merkle_hash<T: Serialize>(data: T) -> H256 {
        hasher(hasher(data)).into()
    }
}
