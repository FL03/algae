/*
    TODO: Update the hashes to match the Blake3 Hash Digests
*/
#[cfg(test)]
/*
    Map(A -> a)
        def.
            This notation abbreviates a is the hash of A; more formally, (A) maps to the hash (a) by the hashing function H
*/
use algae_merkle::{concat_and_hash, is_merkle_valid, MerkleTree};
use decanter::prelude::{Hashable, H256};
use hex_literal::hex;
use rs_merkle::Hasher;

// lazy_static::lazy_static!(
//     static ref SAMPLE_DATA: Vec<H256> = hash_leaves();
// );

#[derive(Clone)]
pub struct BSha256;

impl Hasher for BSha256 {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> Self::Hash {
        blake3::hash(data).into()
    }
}

fn get_merkle_tree_size(leafs: usize) -> usize {
    let mut size = leafs + (leafs % 2);
    let mut l = leafs;
    while l > 1 {
        l = (l as f64 / 2_f64).ceil() as usize;
        size += l;
    }
    size
}

fn get_merkle_depth(leafs: usize) -> usize {
    let mut depth = 1;
    let mut l = leafs;
    while l > 1 {
        l = (l as f64 / 2_f64).ceil() as usize;
        depth += 1;
    }
    depth
}

macro_rules! hash_leaves {
    ($leaves:expr) => {
        $leaves
            .iter()
            .map(|leaf| leaf.hash())
            .collect::<Vec<H256>>()
    };
}

macro_rules! gen_merkle_tree_data {
    () => {{
        vec![
            (hex!("17762fddd969a453925d65717ac3eea21320b66b54342fde15128d6caf21215f")).into(),
            (hex!("10e5cf3d3c8a4f9f3468c8cc58eea84892a22fdadbc1acb22410190044c1d553")).into(),
        ]
    }};
}

macro_rules! gen_merkle_tree_data2 {
    () => {{
        vec![
            (hex!("17762fddd969a453925d65717ac3eea21320b66b54342fde15128d6caf21215f")).into(),
            (hex!("10e5cf3d3c8a4f9f3468c8cc58eea84892a22fdadbc1acb22410190044c1d553")).into(),
            (hex!("ea7aa1fc9efdbe106dbb70369a75e9671fa29d52bd55536711bf197477b8f021")).into(),
            (hex!("d5ede538f628f687e5e0422c7755b503653de2dcd7053ca8791afa5d4787d843")).into(),
            (hex!("27bb492e108bf5e9c724176d7ae75d4cedc422fe4065020bd6140c3fcad3a9e7")).into(),
        ]
    }};
}

/*
    A -> a: ("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d", "b69566be6e1720872f73651d1851a0eae0060a132cf0f64a0ffaea248de6cba0")
    B -> b: ("0101010101010101010101010101010101010101010101010101010101010202", "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f")
    C -> c: (concat(a, b), 6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920") where a ahead of b
*/
#[test]
fn test_merkle_root() {
    let sample = ["a", "b", "c", "d"];
    let leaves = hash_leaves!(sample);
    let nleafs = leaves.len();
    let exp = {
        let a = concat_and_hash(&leaves[0], Some(&leaves[1]));
        let b = concat_and_hash(&leaves[2], Some(&leaves[3]));
        concat_and_hash(&a, Some(&b))
        // add_hash(&a, &leaves[2])
    };

    let a = MerkleTree::from(sample.as_slice());
    let b = rs_merkle::MerkleTree::<BSha256>::from_leaves(sample.iter().map(|i| BSha256::hash(i.as_bytes())).collect::<Vec<[u8; 32]>>().as_slice());

    assert_eq!(a.root(), exp);
    assert_eq!(a.root().0, b.root().expect("No Root"));
    assert_eq!(
        a.dim().shape(),
        (
            get_merkle_depth(nleafs),
            nleafs,
            get_merkle_tree_size(nleafs)
        )
    );
}

#[test]
fn test_merkle_shape() {
    let sample = ["a", "b", "c", "d", "e", "f", "g"];
    let leafs = sample.len();
    let a = MerkleTree::from(sample.as_slice());

    assert_eq!(
        a.dim().shape(),
        (get_merkle_depth(leafs), leafs, get_merkle_tree_size(leafs))
    );
}

/*
    A -> a: ("0101010101010101010101010101010101010101010101010101010101010202", "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f")
*/
#[test]
fn test_merkle_proof() {
    let sample = ["a", "b", "c", "d"];
    let leaves = hash_leaves!(sample);

    let exp = vec![leaves[1], concat_and_hash(&leaves[2], Some(&leaves[3]))];

    let a = MerkleTree::from(leaves.as_slice());

    assert_eq!(a.proof(0), exp);
}

/*
    A -> a: ("0101010101010101010101010101010101010101010101010101010101010202", "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f")
*/
#[test]
fn test_is_merkle_valid() {
    let data: Vec<H256> = gen_merkle_tree_data2!();
    let merkle_tree = MerkleTree::from(data.as_slice());
    let index = 3;
    let proof = merkle_tree.proof(index);
    log::info!("{:?}", proof);
    assert!(is_merkle_valid(
        &merkle_tree.root(),
        &data[index].hash(),
        &proof,
        index,
        data.len()
    ));
}

#[test]
fn test_vrf_tree() {
    let data: Vec<H256> = gen_merkle_tree_data!();
    let merkle_tree = MerkleTree::from(data.as_slice());
    let proof = merkle_tree.proof(0);
    assert!(is_merkle_valid(
        &merkle_tree.root(),
        &data[0].hash(),
        &proof,
        0,
        data.len()
    ));
}
