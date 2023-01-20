/*
    Appellation: misc <utils>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use decanter::prelude::{hasher, H256};
use serde::Serialize;

///
pub fn add_hash(a: &H256, b: &H256) -> H256 {
    let c = [a.as_ref(), b.as_ref()].concat();
    let combined = ring::digest::digest(&ring::digest::SHA256, &c);
    <H256>::from(combined)
}

/// Merges two hashes into a string
pub fn combine<T: ToString>(a: &T, b: &T) -> String {
    format!("{}{}", a.to_string(), b.to_string())
}

/// Takes the hash of the given information to the second degree
pub fn merkle_hash<T: Serialize + ToString>(data: T) -> H256 {
    let res: H256 = {
        let tmp: H256 = hasher(&data).as_slice().to_owned().into();
        hasher(&tmp).as_slice().to_owned().into()
    };
    res
}
