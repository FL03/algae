/*
    Appellation: merkle <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Implements the expected leaf element for composing merkle trees
*/
use super::{nodes::MerkleNode, merkle_hash};
use scsys::crypto::hashes::{H256, Hashable};
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Data<T: ToString>(T);


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Leaf<T: ToString>(Data<T>);






