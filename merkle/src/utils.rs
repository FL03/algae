/*
    Appellation: generate <utils>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use crate::proofs::proof_path;
use crate::{MerkleDimension, MerkleShape};
use decanter::prelude::{hasher, Hashable, H256};

/// Combines two hashes into a single hash
pub fn add_hash(a: &H256, b: &H256) -> H256 {
    let c = [a.as_ref(), b.as_ref()].concat();
    let combined = ring::digest::digest(&ring::digest::SHA256, &c);
    hasher(combined).into()
}
/// Merges two hashes into a string
pub fn combine_hash_str<T: ToString>(a: &T, b: &T) -> String {
    format!("{}{}", a.to_string(), b.to_string())
}
/// Creates a Merkle tree from a slice of data
pub fn create_merkle_tree<T>(data: &[T]) -> (Box<dyn MerkleShape>, Vec<H256>)
where
    T: Hashable,
{
    let mut length = data.len();
    let mut nodes = Vec::new();
    let mut last_level = Vec::new();
    for i in data {
        let h: H256 = i.hash();
        last_level.push(h);
        nodes.push(h);
    }
    let mut depth = 1;
    while length > 1 {
        if length % 2 != 0 {
            last_level.push(data[length - 1].hash());
            nodes.push(data[length - 1].hash());
            length += 1;
        }
        let mut temp = Vec::new();
        for i in 0..length / 2 {
            let h: H256 = add_hash(&last_level[2 * i], &last_level[2 * i + 1]);
            temp.push(h);
            nodes.push(h);
        }
        last_level = temp.clone();
        length /= 2;
        depth += 1;
    }
    let dim = MerkleDimension::new(depth, data.len(), nodes.len());
    (Box::new(dim), nodes)
}

/// Takes the hash of the given information to the second degree
pub fn merkle_hash(data: impl AsRef<[u8]>) -> H256 {
    let tmp: H256 = hasher(data).into();
    hasher(&tmp).into()
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
