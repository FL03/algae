/*
    Appellation: payloads <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use decanter::prelude::Hashable;

#[derive(
    Clone, Debug, Default, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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

impl core::fmt::Display for Position {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {})", self.index(), self.height())
    }
}
