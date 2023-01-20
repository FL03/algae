/*
    Appellation: validate <utils>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use crate::{add_hash, proofs::proof_path};
use decanter::prelude::H256;

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
