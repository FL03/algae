/*
    Appellation: directed <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{
    cmp::{AdjacencyTable, Node},
    Graph, Subgraph,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DirectedGraph<N: Node = String, V: Clone + PartialEq = i64> {
    adjacency_table: AdjacencyTable<N, V>,
}

impl<N: Node, V: Clone + PartialEq> Graph<N, V> for DirectedGraph<N, V> {
    fn new() -> Self {
        Self {
            adjacency_table: AdjacencyTable::new(),
        }
    }
    fn adjacency_table_mut(&mut self) -> &mut AdjacencyTable<N, V> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &AdjacencyTable<N, V> {
        &self.adjacency_table
    }
    fn with_capacity(capacity: usize) -> Self {
        Self {
            adjacency_table: AdjacencyTable::with_capacity(capacity),
        }
    }
}

impl<N: Node, V: Clone + PartialEq> Subgraph<N, V> for DirectedGraph<N, V> {}

impl<N: Node, V: Clone + PartialEq> From<AdjacencyTable<N, V>> for DirectedGraph<N, V> {
    fn from(adjacency_table: AdjacencyTable<N, V>) -> Self {
        Self { adjacency_table }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmp::Edge;

    const TEST_EDGES: [(&str, &str, usize); 3] = [("a", "b", 5), ("c", "a", 7), ("b", "c", 10)];

    #[test]
    fn test_add_node() {
        let mut graph = DirectedGraph::<&str, i64>::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(graph.nodes(), ["a", "b", "c"].iter().cloned().collect());
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DirectedGraph::new();

        for i in TEST_EDGES {
            graph.add_edge(i.into());
        }
        for edge in TEST_EDGES {
            assert!(graph.contains_edge(&Edge::from(edge)));
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph = DirectedGraph::new();

        for i in TEST_EDGES {
            graph.add_edge(i.into());
        }

        assert_eq!(graph.neighbours("a").unwrap(), &vec![("b", 5)]);
    }

    #[test]
    fn test_contains() {
        let mut graph = DirectedGraph::<&str, i64>::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert!(graph.contains_node(&"a"));
        assert!(graph.contains_node(&"b"));
        assert!(graph.contains_node(&"c"));
        assert!(!graph.contains_node(&"d"));
    }
}
