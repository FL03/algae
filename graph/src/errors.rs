/*
   Appellation: errors <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description:
*/
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum Errors {
    #[default]
    NodeNotInGraph,
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.clone() {
            Self::NodeNotInGraph => write!(f, "accessing a node that is not in the graph"),
        }
    }
}
