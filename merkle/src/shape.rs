/*
   Appellation: shape <merkle>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};

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

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct MerkleDimension {
    depth: usize,
    leafs: usize,
    size: usize,
}

impl MerkleDimension {
    pub fn new(depth: usize, leafs: usize, size: usize) -> Self {
        Self { depth, leafs, size }
    }

    pub fn from_leafs(leafs: usize) -> Self {
        let depth = get_merkle_depth(leafs);
        let size = get_merkle_tree_size(leafs);

        Self::new(depth, leafs, size)
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn leafs(&self) -> usize {
        self.leafs
    }

    pub fn shape(&self) -> (usize, usize, usize) {
        (self.depth, self.leafs, self.size)
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl std::fmt::Display for MerkleDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.depth, self.leafs, self.size)
    }
}

impl From<(usize, usize, usize)> for MerkleDimension {
    fn from(data: (usize, usize, usize)) -> Self {
        Self::new(data.0, data.1, data.2)
    }
}

impl From<MerkleDimension> for (usize, usize, usize) {
    fn from(data: MerkleDimension) -> Self {
        (data.depth, data.leafs, data.size)
    }
}
