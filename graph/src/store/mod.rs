/*
    Appellation: store <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
pub use self::{matrix::*, table::*};

pub(crate) mod matrix;
pub(crate) mod table;

use crate::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Stores<N, V>
where
    N: Node,
    V: Clone + PartialEq,
{
    Matrix(AdjacencyMatrix<N, V>),
    Table(AdjacencyTable<N, V>),
}
