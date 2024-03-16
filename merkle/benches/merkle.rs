// bench.rs
#![feature(test)]

extern crate test;
use algae_merkle::MerkleTree;
use decanter::prelude::{Hashable, H256};
use rs_merkle::Hasher;
use test::Bencher;

const TEST_NODES: [&str; 4] = ["a", "b", "c", "d"];

#[bench]
fn bench_merkle_proofs(b: &mut Bencher) {
    let leafs = TEST_NODES.iter().map(|i| i.hash()).collect::<Vec<H256>>();
    let tree = MerkleTree::from(leafs.as_slice());
    b.iter(|| {
        leafs
            .iter()
            .enumerate()
            .for_each(|(i, leaf)| assert!(!tree.proof(i).contains(leaf)));
    });
}

#[derive(Clone)]
pub struct BSha256Algorithm;

impl Hasher for BSha256Algorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> Self::Hash {
        let mut hasher = blake3::Hasher::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

#[bench]
fn bench_rs_merkle(b: &mut Bencher) {
    use rs_merkle::Hasher;
    use rs_merkle::MerkleTree as RSMerkleTree;

    let leafs = TEST_NODES
        .iter()
        .map(|i| BSha256Algorithm::hash(i.as_bytes()))
        .collect::<Vec<[u8; 32]>>();
    let tree = RSMerkleTree::<BSha256Algorithm>::from_leaves(&leafs);
    b.iter(|| {
        leafs
            .iter()
            .enumerate()
            .for_each(|(i, leaf)| assert!(!tree.proof(&[i]).proof_hashes().contains(leaf)));
    });
}
