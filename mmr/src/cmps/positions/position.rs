/*
    Appellation: payloads <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Position {
    pub height: usize,
    pub index: usize,
}

impl Position {
    pub fn new(height: usize, index: usize) -> Self {
        Self { height, index }
    }
}

impl From<(usize, usize)> for Position {
    fn from(data: (usize, usize)) -> Self {
        Self::new(data.0, data.1)
    }
}

impl Hashable for Position {
    fn hash(&self) -> H256 {
        hasher(&self).as_slice().to_owned().into()
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
