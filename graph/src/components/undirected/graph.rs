/*
   Appellation: graph <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description:
*/
use crate::{AdjacencyHashTable, Graphable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UndirectedGraph {
    adjacency_table: AdjacencyHashTable,
}

impl Graphable for UndirectedGraph {
    fn adjacency_table_mutable(&mut self) -> &mut AdjacencyHashTable {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &AdjacencyHashTable {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_table
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
        self.adjacency_table
            .entry(edge.1.to_string())
            .and_modify(|e| {
                e.push((edge.0.to_string(), edge.2));
            });
    }
}
