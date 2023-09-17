// bench.rs
#![feature(test)]

extern crate test;
use algae_merkle::MerkleTree;
use decanter::prelude::{H256, Hashable};

use test::Bencher;

const TEST_NODES: [&str; 4] = ["a", "b", "c", "d"];

#[bench]
fn bench_merkle_proofs(b: &mut Bencher) {
    let leafs = TEST_NODES
        .iter()
        .map(|i| i.hash())
        .collect::<Vec<H256>>();
    let tree = MerkleTree::from(leafs.as_slice());
    b.iter(|| {
        leafs.iter().enumerate().for_each(|(i, leaf)| assert!(!tree.proof(i).contains(leaf)));
    });
}

#[bench]
fn bench_rs_merkle(b: &mut Bencher) {
    use rs_merkle::MerkleTree as RSMerkleTree;
    use rs_merkle::algorithms::Sha256;
    use rs_merkle::Hasher;

    let leafs = TEST_NODES
        .iter()
        .map(|i| Sha256::hash(i.as_bytes()))
        .collect::<Vec<[u8; 32]>>();
    let tree = RSMerkleTree::<Sha256>::from_leaves(&leafs);
    b.iter(|| {
        leafs.iter().enumerate().for_each(|(i, leaf)| assert!(!tree.proof(&[i]).proof_hashes().contains(leaf)));
    });
}

