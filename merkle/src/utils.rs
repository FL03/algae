/*
    Appellation: generate <utils>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use crate::proofs::proof_path;
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

/// Verify that the datum hash with a vector of proofs will produce the Merkle root. Also need the
/// index of datum and `leaf_size`, the total number of leaves.
pub fn is_merkle_valid(
    root: &H256,
    datum: &H256,
    proof: &[H256],
    index: usize,
    leaf_size: usize,
) -> bool {
    let mut h: H256 = *datum;
    let proof_index = proof_path(index, leaf_size);
    for i in 0..proof.len() {
        if proof_index[i] % 2 == 0 {
            h = add_hash(&proof[i], &h);
        } else {
            h = add_hash(&h, &proof[i]);
        }
    }
    *root == h
}
