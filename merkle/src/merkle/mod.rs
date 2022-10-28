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
    use crate::{crypto::hash::Hash, join};
    use std::string::ToString;

    pub fn merkle_hash<T: ToString>(data: T) -> String {
        Hash::from(Hash::new(data)).to_string()
    }

    pub fn combine<T: ToString>(a: &T, b: &T) -> String {
        join![a.to_string().as_str(), b.to_string().as_str()]
    }
}
