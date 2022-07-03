use algae;

#[cfg(test)]
mod tests {
    #[test]
    fn test_btree_search() {
        let values = [10, 20, 30, 5, 6, 7, 11, 12, 15];
        let mut tree = algae::structures::BTree::new(2);
        for i in 0..values.len() {
            tree.insert(values[i])
        }
        assert!(tree.search(15));
        assert_eq!(tree.search(16), false);
    }
}
