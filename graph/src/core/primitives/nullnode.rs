/*
   Appellation: nullnode <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description:
*/
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct NodeNotInGraph;

impl std::fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
