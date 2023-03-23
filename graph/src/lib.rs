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
pub mod search;
pub mod store;

use cmp::{Edge, Node};
use errors::GraphError;
use std::collections::HashSet;
use store::AdjacencyTable;

pub trait Contain<T> {
    /// [Contain::contains] returns true if the given element is in the [Contain] instance
    fn contains(&self, elem: &T) -> bool;
    /// [Contain::contains_many] returns true if all elements in the given iterator are in the [Contain] instance
    fn contains_many(&self, iter: impl IntoIterator<Item = T>) -> bool {
        iter.into_iter().all(|i| self.contains(&i))
    }
    /// [Contain::contains_some] returns true if any element in the given iterator is in the [Contain] instance
    fn contains_some(&self, iter: impl IntoIterator<Item = T>) -> bool {
        iter.into_iter().any(|i| self.contains(&i))
    }
}

/// [Graph] describes the basic operations of a graph data-structure
pub trait Graph<N = String, V = i64>: Clone + Contain<N> + Contain<Edge<N, V>>
where
    N: Node,
    V: Clone + PartialEq,
{
    fn new() -> Self;
    /// [Graph::add_edge] inserts a new [Edge] into the graph
    fn add_edge(&mut self, edge: Edge<N, V>) {
        let pair = edge.pair();
        self.add_node(pair.0.clone());
        self.add_node(pair.1.clone());

        self.store_mut().entry(pair.0.clone()).and_modify(|e| {
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
        match self.store().get(&node) {
            None => {
                self.store_mut().insert(node, Vec::new());
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
    /// [Graph::store_mut] returns an owned, mutable instance of the [AdjacencyTable]
    fn store_mut(&mut self) -> &mut AdjacencyTable<N, V>;
    /// [Graph::store] returns an owned instance of the [AdjacencyTable]
    fn store(&self) -> &AdjacencyTable<N, V>;
    /// [Graph::edges] returns all of the edges persisting within the graph
    fn edges(&self) -> Vec<Edge<N, V>> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.store().clone() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node.clone(), to_node, weight).into());
            }
        }
        edges
    }
    /// [Graph::neighbors] attempts to return a [Vec] that contains all of the connected [Node] and their values
    fn neighbors(&self, node: N) -> Result<&Vec<(N, V)>, GraphError> {
        match self.store().get(&node) {
            None => Err(GraphError::NodeNotInGraph),
            Some(i) => Ok(i),
        }
    }
    /// [Graph::nodes] returns a cloned [HashSet] of the graph's current [Node]s
    fn nodes(&self) -> HashSet<N> {
        self.store().keys().cloned().collect()
    }
    /// [Graph::with_capacity] is a method for creating a graph with a set number of nodes
    fn with_capacity(capacity: usize) -> Self;
}

pub trait Subgraph<N: Node = String, V: Clone + PartialEq = i64>: Graph<N, V> {
    fn is_subgraph(&self, graph: impl Graph<N, V>) -> bool {
        self.nodes().is_subset(&graph.nodes())
    }
}
