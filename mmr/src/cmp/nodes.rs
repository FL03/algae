/*
    Appellation: cmps <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use decanter::prelude::{Hashable, H256};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Node {
    pub hash: H256,
    pub pruned: bool,
}

impl Node {
    pub fn new(hash: H256, pruned: bool) -> Node {
        Node { hash, pruned }
    }
    pub fn prune(&mut self) {
        self.pruned = true;
    }
    pub fn is_pruned(&self) -> bool {
        self.pruned
    }
}

impl Hashable for Node {
    fn hash(&self) -> H256 {
        self.hash
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hash)
    }
}

impl From<H256> for Node {
    fn from(data: H256) -> Self {
        Self::new(data, false)
    }
}
