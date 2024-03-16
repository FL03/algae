/*
    Appellation: builder <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub struct MerkleTreeBuilder<T> {
    leafs: Vec<T>,
}

impl<T> MerkleTreeBuilder<T> {
    pub fn new() -> Self {
        Self { leafs: Vec::new() }
    }

    pub fn with_leafs(mut self, leafs: Vec<T>) -> Self {
        self.leafs = leafs;
        self
    }
}
