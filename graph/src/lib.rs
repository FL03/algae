/*
    Appellation: graphs <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This library is dedicated to graphs, explicitly implementing generic directed and undirected data-structures while providing the tools to create new ones.
*/
pub use self::{directed::*, undirected::*};

pub(crate) mod directed;
pub(crate) mod undirected;

pub mod cmp;
pub mod errors;

use cmp::{AdjacencyTable, Edge, Node};
use errors::GraphError;
use std::collections::HashSet;

/// [Graph] describes the basic operations of a graph data-structure
pub trait Graph<N: Node = String, V: Clone + PartialEq = i64>: Clone {
    fn new() -> Self;
    /// [Graph::add_edge] inserts a new [Edge] into the graph
    fn add_edge(&mut self, edge: Edge<N, V>) {
        let pair = edge.pair();
        self.add_node(pair.0.clone());
        self.add_node(pair.1.clone());

        self.adjacency_table_mut()
            .entry(pair.0.clone())
            .and_modify(|e| {
                e.push((pair.1, edge.value()));
            });
    }
    /// [Graph::add_edges] insert several edges into the graph
    fn add_edges(&mut self, iter: impl IntoIterator<Item = Edge<N, V>>) {
        for i in iter {
            self.add_edge(i)
        }
    }
    /// [Graph::add_node] if the given [Node] is not already in the [Graph], insert the [Node] and return true; else return false
    fn add_node(&mut self, node: N) -> bool {
        match self.adjacency_table().get(&node) {
            None => {
                self.adjacency_table_mut().insert(node, Vec::new());
                true
            }
            _ => false,
        }
    }
    /// [Graph::add_nodes] insert several nodes into the graph
    fn add_nodes(&mut self, iter: impl IntoIterator<Item = N>) {
        for i in iter {
            self.add_node(i);
        }
    }
    /// [Graph::adjacency_table_mut] returns an owned, mutable instance of the [AdjacencyTable]
    fn adjacency_table_mut(&mut self) -> &mut AdjacencyTable<N, V>;
    /// [Graph::adjacency_table] returns an owned instance of the [AdjacencyTable]
    fn adjacency_table(&self) -> &AdjacencyTable<N, V>;
    /// [Graph::contains_edge] checks to see if a given [Edge] is found in the [Graph]
    fn contains_edge(&self, edge: &Edge<N, V>) -> bool {
        self.edges().contains(edge)
    }
    /// [Graph::contains_node] checks to see if a given [Node] is found in the [Graph]
    fn contains_node(&self, node: &N) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    /// [Graph::edges] returns all of the edges persisting within the graph
    fn edges(&self) -> Vec<Edge<N, V>> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table().clone() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node.clone(), to_node, weight).into());
            }
        }
        edges
    }
    /// [Graph::neighbours] attempts to return a [Vec] that contains all of the connected [Node] and their values
    fn neighbours(&self, node: N) -> Result<&Vec<(N, V)>, GraphError> {
        match self.adjacency_table().get(&node) {
            None => Err(GraphError::NodeNotInGraph),
            Some(i) => Ok(i),
        }
    }
    /// [Graph::nodes] returns a cloned [HashSet] of the graph's current [Node]s
    fn nodes(&self) -> HashSet<N> {
        self.adjacency_table().keys().cloned().collect()
    }
    /// [Graph::with_capacity] is a method for creating a graph with a set number of nodes
    fn with_capacity(capacity: usize) -> Self;
}

pub trait Subgraph<N: Node = String, V: Clone + PartialEq = i64>: Graph<N, V> {
    fn is_subgraph(&self, graph: impl Graph<N, V>) -> bool {
        self.nodes().is_subset(&graph.nodes())
    }
}
