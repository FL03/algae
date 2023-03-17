/*
    Appellation: undirected <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{
    cmp::{AdjacencyTable, Edge, Node},
    Graph, Subgraph,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UndirectedGraph<N: Node = String, V: Clone + PartialEq = i64> {
    adjacency_table: AdjacencyTable<N, V>,
}

impl<N: Node, V: Clone + PartialEq> Graph<N, V> for UndirectedGraph<N, V> {
    fn new() -> Self {
        Self {
            adjacency_table: AdjacencyTable::new(),
        }
    }
    fn add_edge(&mut self, edge: Edge<N, V>) {
        let pair = edge.pair();
        self.add_node(pair.0.clone());
        self.add_node(pair.1.clone());

        self.adjacency_table.entry(pair.0.clone()).and_modify(|e| {
            e.push((pair.1.clone(), edge.value()));
        });
        self.adjacency_table.entry(pair.1).and_modify(|e| {
            e.push((pair.0, edge.value()));
        });
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

impl<N: Node, V: Clone + PartialEq> Subgraph<N, V> for UndirectedGraph<N, V> {}

impl<N: Node, V: Clone + PartialEq> From<AdjacencyTable<N, V>> for UndirectedGraph<N, V> {
    fn from(adjacency_table: AdjacencyTable<N, V>) -> Self {
        Self { adjacency_table }
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;
    use super::UndirectedGraph;
    use crate::cmp::Edge;

    const TEST_EDGES: [(&str, &str, usize); 3] = [("a", "b", 5), ("c", "a", 7), ("b", "c", 10)];

    const EXPECTED: [(&str, &str, usize); 6] = [
        ("a", "b", 5),
        ("b", "a", 5),
        ("c", "a", 7),
        ("a", "c", 7),
        ("b", "c", 10),
        ("c", "b", 10),
    ];

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();

        for i in TEST_EDGES {
            graph.add_edge(i.into());
        }

        for edge in EXPECTED {
            assert!(graph.edges().contains(&Edge::from(edge)));
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph = UndirectedGraph::new();

        for i in TEST_EDGES {
            graph.add_edge(i.into());
        }

        assert_eq!(graph.neighbours("a").unwrap(), &vec![("b", 5), ("c", 7)]);
    }
}
