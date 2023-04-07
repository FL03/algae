/*
    Appellation: payloads <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::prelude::Hashable;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Position {
    pub height: usize,
    pub index: usize,
}

impl Position {
    pub fn new(height: usize, index: usize) -> Self {
        Self { height, index }
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn index(&self) -> usize {
        self.index
    }
}

impl From<(usize, usize)> for Position {
    fn from(data: (usize, usize)) -> Self {
        Self::new(data.0, data.1)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
