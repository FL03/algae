/*
   Appellation: algae-merkle <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
*/
pub use self::{merkle::MerkleTree, utils::*};

pub mod components;
pub(crate) mod merkle;
pub mod mmr;
pub mod proofs;

pub(crate) mod utils {
    use scsys::crypto::hash::{H256, Hashable, Hash};
    use std::string::ToString;

    pub fn merkle_hash<T: ToString>(data: T) -> H256 {
        let res: Vec<u8> = Hash::from(Hash::new(data)).into();
        res.into()
    }

    pub fn combine<T: ToString>(a: &T, b: &T) -> String {
        format!("{}{}", a.to_string(), b.to_string())
    }
}
