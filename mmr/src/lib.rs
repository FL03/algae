/*
   Appellation: algae-mmr <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
*/
pub use self::{core::*, utils::*};

pub(crate) mod core;
pub mod proofs;

pub(crate) mod utils {
    use scsys::crypto::hash::{Hash, H256};
    use std::string::ToString;

    pub fn merkle_hash<T: ToString>(data: T) -> H256 {
        let res: Vec<u8> = Hash::new(data).into();
        res.into()
    }

    pub fn combine<T: ToString>(a: &T, b: &T) -> String {
        format!("{}{}", a.to_string(), b.to_string())
    }
}
