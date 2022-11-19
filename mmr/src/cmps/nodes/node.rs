/*
    Appellation: cmps <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

use scsys::prelude::H256;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct MerkleNode {
    pub hash: H256,
    pub pruned: bool,
}

impl MerkleNode {
    pub fn new(hash: H256, pruned: bool) -> MerkleNode {
        MerkleNode { hash, pruned }
    }
}

impl From<H256> for MerkleNode {
    fn from(data: H256) -> Self {
        Self::new(data, false)
    }
}
