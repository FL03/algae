/*
   Appellation: shape <merkle>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};


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

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn leafs(&self) -> usize {
        self.leafs
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
