/*
    Appellation: merkle <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This module is dedicated towards the implementation of a complete, WebAssembley native merkle tree for blockchain's and other advanced applications
*/
pub use self::{nodes::*, tree::*, utils::*};

pub(crate) mod nodes;
pub(crate) mod tree;

pub(crate) mod utils {
    use super::nodes::{MerkleNode, MerklePayload};
    use sha2::{Digest, Sha256};
    use std::string::ToString;

    pub fn combine<T: ToString>(a: &T, b: &T) -> String {
        format!("{}{}", a.to_string(), b.to_string())
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

    pub fn merkle_hash<T: ToString>(data: T) -> String {
        compute_hash(compute_hash(data))
    }
}
