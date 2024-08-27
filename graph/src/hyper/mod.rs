/*
    Appellation: hyper <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # HyperGraphs
//! 
//! 
#[doc(inline)]
pub use self::graph::HyperGraph;

mod graph;

pub(crate) mod prelude {
    pub use crate::graph::HyperGraph;
}