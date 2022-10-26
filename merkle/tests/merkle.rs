#[cfg(test)]
mod tests {
    use algae_merkle::*;

    #[test]
    fn test_equal_content_has_equal_merkle_root() {
        let content1 = vec![
            "TX:7194AB90DFCC;SENDER:AF5B20CD94;RECIPIENT:992459AFB9;AMOUNT:100.0",
            "TX:7194AB90EFBA;SENDER:BD89AA8209;RECIPIENT:5892AB98EF;AMOUNT:87.58",
            "TX:7194AB91D7B5;SENDER:C2A79CA87F;RECIPIENT:24814DD2E9;AMOUNT:145.72",
            "TX:7194AB91EDDE;SENDER:D20E51D80C;RECIPIENT:37A2D3B07B;AMOUNT:185.13",
            "TX:7194AB92C71B;SENDER:8F61DFCF27;RECIPIENT:CBD23E833A;AMOUNT:108.81",
            "TX:7194AB9259A1;SENDER:9CFD5BEAAD;RECIPIENT:3107353190;AMOUNT:125.96",
            "TX:7194AB9217F4;SENDER:A818663B1A;RECIPIENT:C04C6B5F44;AMOUNT:131.86",
            "TX:7194AB93AA95;SENDER:ACC3EEA3B4;RECIPIENT:28DB574BD0;AMOUNT:176.20",
        ];

        let content2 = content1.clone();
        let tree1 = merkle::MerkleTree::<&str>::from(content1);
        let tree2 = merkle::MerkleTree::<&str>::from(content2);
        assert_eq!(tree1.root_hash(), tree2.root_hash());
    }

    #[test]
    fn test_different_content_produces_different_merkle_root() {
        let content1 = vec![
            "TX:7194AB90DFCC;SENDER:AF5B20CD94;RECIPIENT:992459AFB9;AMOUNT:100.0",
            "TX:7194AB90EFBA;SENDER:BD89AA8209;RECIPIENT:5892AB98EF;AMOUNT:87.58",
            "TX:7194AB91D7B5;SENDER:C2A79CA87F;RECIPIENT:24814DD2E9;AMOUNT:145.72",
            "TX:7194AB91EDDE;SENDER:D20E51D80C;RECIPIENT:37A2D3B07B;AMOUNT:185.13",
            "TX:7194AB92C71B;SENDER:8F61DFCF27;RECIPIENT:CBD23E833A;AMOUNT:108.81",
            "TX:7194AB9259A1;SENDER:9CFD5BEAAD;RECIPIENT:3107353190;AMOUNT:125.96",
            "TX:7194AB9217F4;SENDER:A818663B1A;RECIPIENT:C04C6B5F44;AMOUNT:131.86",
            "TX:7194AB93AA95;SENDER:ACC3EEA3B4;RECIPIENT:28DB574BD0;AMOUNT:176.20",
        ];

        let content2 = vec![
            "TX:7194AB90DFCC;SENDER:AF5B20CD94;RECIPIENT:992459AFB9;AMOUNT:100.0",
            "TX:7194AB90EFBA;SENDER:BD89AA8209;RECIPIENT:5892AB98EF;AMOUNT:87.58",
            "TX:7194AB91D7B5;SENDER:C2A79CA87F;RECIPIENT:24814DD2E9;AMOUNT:145.72",
            "TX:7194AB91EDDE;SENDER:D20E51D80C;RECIPIENT:37A2D3B07B;AMOUNT:185.13",
            "TX:7194AB92C71B;SENDER:8F61DFCF27;RECIPIENT:CBD23E833A;AMOUNT:108.81",
            "TX:7194AB9259A1;SENDER:9CFD5BEAAD;RECIPIENT:3107353190;AMOUNT:125.96",
            "TX:7194AB9217F4;SENDER:A818663B1A;RECIPIENT:C04C6B5F44;AMOUNT:10000.00", // <-- this transaction has altered amount
            "TX:7194AB93AA95;SENDER:ACC3EEA3B4;RECIPIENT:28DB574BD0;AMOUNT:176.20",
        ];

        let tree1 = merkle::MerkleTree::<&str>::from(content1);
        let tree2 = merkle::MerkleTree::<&str>::from(content2);
        assert!(tree1.root_hash() != tree2.root_hash());
    }
}
