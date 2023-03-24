/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: components (cmp) for building effecient graph data-structures
*/
pub use self::{edge::*, neighbors::*, pair::*};

pub(crate) mod edge;
pub(crate) mod neighbors;
pub(crate) mod pair;
