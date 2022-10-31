/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

use scsys::crypto::hash::{Hash, Hashable, H256};
use std::string::ToString;

pub fn merkle_hash<T: ToString>(data: T) -> H256 {
    let res: Vec<u8> = Hash::from(Hash::new(data)).into();
    res.into()
}

pub fn combine<T: ToString>(a: &T, b: &T) -> String {
    format!("{}{}", a.to_string(), b.to_string())
}