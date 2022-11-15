/*
   Appellation: graph <module> [directed]
   Contrib: FL03 <jo3mccain@icloud.com>
   Description:
*/
use crate::{AdjacencyHashTable, Graphable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DirectedGraph {
    adjacency_table: AdjacencyHashTable,
}

impl Graphable for DirectedGraph {
    fn adjacency_table_mutable(&mut self) -> &mut AdjacencyHashTable {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &AdjacencyHashTable {
        &self.adjacency_table
    }
}
