/*

*/
#[cfg(test)]
mod tests {
    /*
       Map(A -> a)
          def.
             This notation abbreviates a is the hash of A; more formally, (A) maps to the hash (a) by the hashing function H
    */
    use algae_merkle::{is_merkle_valid, MerkleTree, MerkleTreeWrapper};
    use hex_literal::hex;
    use scsys::prelude::{Hashable, H256};

    macro_rules! gen_merkle_tree_data {
        () => {{
            vec![
                (hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
            ]
        }};
    }

    macro_rules! gen_merkle_tree_data2 {
        () => {{
            vec![
                (hex!("0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d0a0b0c0d0e0f0e0d")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
                (hex!("0101010101010101010101010101010101010101010101010101010101010202")).into(),
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
        let data: Vec<H256> = gen_merkle_tree_data!();
        let expected =
            (hex!("6b787718210e0b3b608814e04e61fde06d0df794319a12162f287412df3ec920")).into();
        let a = MerkleTree::from(data);
        assert_eq!(&a.root(), &expected);
    }

    /*
      A -> a: ("0101010101010101010101010101010101010101010101010101010101010202", "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f")
    */
    #[test]
    fn test_merkle_proof() {
        let expected =
            vec![hex!("965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f").into()];

        let a = MerkleTree::from(gen_merkle_tree_data!());

        assert_eq!(a.proof(0), expected)
    }

    /*
       A -> a: ("0101010101010101010101010101010101010101010101010101010101010202", "965b093a75a75895a351786dd7a188515173f6928a8af8c9baa4dcff268a4f0f")
    */
    #[test]
    fn test_is_merkle_valid() {
        let data: Vec<H256> = gen_merkle_tree_data2!();
        let merkle_tree = MerkleTree::from(data.clone());
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
        let merkle_tree = MerkleTree::from(data.clone());
        let proof = merkle_tree.proof(0);
        assert!(is_merkle_valid(
            &merkle_tree.root(),
            &data[0].hash(),
            &proof,
            0,
            data.len()
        ));
    }
}
