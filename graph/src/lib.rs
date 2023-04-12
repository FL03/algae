/*
    Appellation: graphs <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This library is dedicated to graphs, explicitly implementing generic directed and undirected data-structures while providing the tools to create new ones.
*/
pub use self::{cmp::*, directed::*, errors::*, specs::*, undirected::*};

pub(crate) mod cmp;
pub(crate) mod directed;
mod errors;
mod specs;
pub(crate) mod undirected;

pub mod graph;
pub mod search;
pub mod store;

use errors::GraphError;
use std::{collections::HashSet, ops::IndexMut};
use store::AdjacencyTable;

/// [Graph] describes the basic operations of a graph data-structure
pub trait Graph<N = String, V = i64>:
    Clone + Contain<N> + Contain<Edge<N, V>> + IndexMut<N, Output = Vec<(N, V)>>
where
    N: Node,
    V: Weight,
{
    /// [Graph::add_edge] inserts a new [Edge] into the graph
    fn add_edge(&mut self, edge: Edge<N, V>) {
        let pair = edge.pair();
        self.add_node(pair.0.clone());
        self.add_node(pair.1.clone());

        self.store_mut().entry(pair.0.clone()).and_modify(|e| {
            e.push((pair.1, edge.value().clone()));
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
    /// [Graph::contains_edge] returns true if the given [Edge] is in the graph
    fn contains_edge(&self, edge: &Edge<N, V>) -> bool {
        match self.store().get(&edge.pair().0) {
            Some(edges) => edges.contains(&(edge.pair().1, edge.value().clone())),
            None => false,
        }
    }
    /// [Graph::contains_node] returns true if the given [Node] is in the graph
    fn contains_node(&self, node: &N) -> bool {
        self.store().contains_key(node)
    }
    /// [Graph::degree] returns the degree of the given [Node]
    fn degree(&self, node: &N) -> Result<usize, GraphError> {
        match self.store().get(node) {
            Some(edges) => Ok(edges.len()),
            None => Err(GraphError::NodeNotInGraph),
        }
    }
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
    /// [Graph::edges_from] returns all of the edges originating from the given [Node]
    fn edges_from(&self, node: &N) -> Result<Vec<Edge<N, V>>, GraphError> {
        match self.store().get(node) {
            Some(edges) => Ok(edges
                .iter()
                .cloned()
                .map(|(n, v)| Edge::new(node.clone(), n, v))
                .collect()),
            None => Err(GraphError::NodeNotInGraph),
        }
    }
    /// [Graph::edges_to] returns all of the edges terminating at the given [Node]
    fn edges_to(&self, node: &N) -> Result<Vec<Edge<N, V>>, GraphError> {
        let mut edges = Vec::new();
        for (origin, neighbours) in self.store().clone() {
            for (dest, weight) in neighbours {
                if &dest == node {
                    edges.push((origin.clone(), dest, weight).into());
                }
            }
        }
        Ok(edges)
    }
    /// [Graph::is_connected] returns true if the graph is connected
    fn is_connected(&self) -> bool {
        let mut visited = HashSet::new();
        let mut stack = self.nodes().iter().cloned().collect::<Vec<_>>();

        while let Some(node) = stack.pop() {
            if !visited.contains(&node) {
                visited.insert(node.clone());
                stack.extend(
                    self.neighbors(node)
                        .unwrap()
                        .iter()
                        .map(|(n, _)| n)
                        .cloned(),
                );
            }
        }

        visited.len() == self.nodes().len()
    }
    /// [Graph::store_mut] returns an owned, mutable instance of the [AdjacencyTable]
    fn store_mut(&mut self) -> &mut AdjacencyTable<N, V>;
    /// [Graph::store] returns an owned instance of the [AdjacencyTable]
    fn store(&self) -> &AdjacencyTable<N, V>;
    /// [Graph::neighbors] attempts to return a [Vec] that contains all of the connected [Node] and their values
    fn neighbors(&self, node: N) -> Result<&Vec<(N, V)>, GraphError> {
        if self.nodes().contains(&node) {
            Ok(&self[node])
        } else {
            Err(GraphError::NodeNotInGraph)
        }
    }
    /// [Graph::nodes] returns a cloned [HashSet] of the graph's current [Node]s
    fn nodes(&self) -> HashSet<N> {
        self.store().keys().cloned().collect()
    }
}

pub trait GraphExt<N = String, V = i64>: Graph<N, V>
where
    N: Node,
    V: Weight,
{
}

pub trait Subgraph<N = String, V = i64>: Graph<N, V>
where
    N: Node,
    V: Weight,
{
    fn is_subgraph(&self, graph: impl Graph<N, V>) -> bool {
        self.nodes().is_subset(&graph.nodes())
    }
}
