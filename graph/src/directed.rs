/*
    Appellation: directed <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{cmp::Edge, store::AdjacencyTable, Contain, Graph, Node, Subgraph};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DirectedGraph<N = String, V = i64>
where
    N: Node,
    V: Clone + PartialEq,
{
    store: AdjacencyTable<N, V>,
}

impl<N: Node, V: Clone + PartialEq> AsMut<AdjacencyTable<N, V>> for DirectedGraph<N, V> {
    fn as_mut(&mut self) -> &mut AdjacencyTable<N, V> {
        &mut self.store
    }
}

impl<N: Node, V: Clone + PartialEq> AsRef<AdjacencyTable<N, V>> for DirectedGraph<N, V> {
    fn as_ref(&self) -> &AdjacencyTable<N, V> {
        &self.store
    }
}

impl<N: Node, V: Clone + PartialEq> Contain<N> for DirectedGraph<N, V> {
    fn contains(&self, elem: &N) -> bool {
        self.store.contains_key(elem)
    }
}

impl<N: Node, V: Clone + PartialEq> Contain<Edge<N, V>> for DirectedGraph<N, V> {
    fn contains(&self, elem: &Edge<N, V>) -> bool {
        self.edges().contains(elem)
    }
}

impl<N: Node, V: Clone + PartialEq> Graph<N, V> for DirectedGraph<N, V> {
    fn new() -> Self {
        Self {
            store: AdjacencyTable::new(),
        }
    }
    fn store_mut(&mut self) -> &mut AdjacencyTable<N, V> {
        &mut self.store
    }
    fn store(&self) -> &AdjacencyTable<N, V> {
        &self.store
    }
    fn with_capacity(capacity: usize) -> Self {
        Self {
            store: AdjacencyTable::with_capacity(capacity),
        }
    }
}

impl<N: Node, V: Clone + PartialEq> Subgraph<N, V> for DirectedGraph<N, V> {}

impl<N: Node, V: Clone + PartialEq> From<AdjacencyTable<N, V>> for DirectedGraph<N, V> {
    fn from(adjacency_table: AdjacencyTable<N, V>) -> Self {
        Self {
            store: adjacency_table,
        }
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
            assert!(graph.contains(&Edge::from(edge)));
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph = DirectedGraph::new();

        for i in TEST_EDGES {
            graph.add_edge(i.into());
        }

        assert_eq!(graph.neighbors("a").unwrap(), &vec![("b", 5)]);
    }

    #[test]
    fn test_contains() {
        let mut graph = DirectedGraph::<&str, i64>::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert!(graph.contains_many(["a", "b", "c"]));
        assert!(graph.contains_some(["a", "b", "c", "d"]));
    }
}
