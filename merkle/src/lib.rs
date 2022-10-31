/*
   Appellation: algae-merkle <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
*/
pub use self::{tree::*, utils::*};

pub mod proofs;
pub(crate) mod tree;

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
