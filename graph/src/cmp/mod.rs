/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: components (cmp) for building effecient graph data-structures
*/
//! # Components (cmp)
//!
//!
pub use self::{edge::*, pair::*};

pub(crate) mod edge;
pub(crate) mod pair;

pub mod entry;

pub(crate) mod prelude {
    pub use super::edge::*;
    pub use super::entry::*;
    pub use super::pair::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge() {
        let a = "a";
        let b = "b";
        let weight = 1;
        let edge = Edge::new(a, b, weight);
        assert_eq!(edge.pair(), Pair::new(a, b));
        assert_eq!(edge.value(), &weight);
    }
}
