/*
   Appellation: specs <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description:
*/
use crate::{AdjacencyHashTable, Errors};
use std::collections::HashSet;

pub trait Graphable: Clone + Default {
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_table_mutable()
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
    }
    fn add_node(&mut self, node: &str) -> bool {
        if !self.contains(node) {
            self.adjacency_table_mutable()
                .insert((*node).to_string(), Vec::new());
            return true;
        }
        false
    }
    fn adjacency_table(&self) -> &AdjacencyHashTable;
    fn adjacency_table_mutable(&mut self) -> &mut AdjacencyHashTable;
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
    fn neighbours(&self, node: &str) -> Result<&Vec<(String, i32)>, Errors> {
        match self.adjacency_table().get(node) {
            None => Err(Errors::NodeNotInGraph),
            Some(i) => Ok(i),
        }
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
}
