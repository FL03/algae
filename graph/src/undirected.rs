/*
    Appellation: undirected <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{cmp::Edge, store::AdjacencyTable, Contain, Graph, Node, Subgraph};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UndirectedGraph<N: Node = String, V: Clone + PartialEq = i64> {
    store: AdjacencyTable<N, V>,
}

impl<N: Node, V: Clone + PartialEq> AsMut<AdjacencyTable<N, V>> for UndirectedGraph<N, V> {
    fn as_mut(&mut self) -> &mut AdjacencyTable<N, V> {
        &mut self.store
    }
}

impl<N: Node, V: Clone + PartialEq> AsRef<AdjacencyTable<N, V>> for UndirectedGraph<N, V> {
    fn as_ref(&self) -> &AdjacencyTable<N, V> {
        &self.store
    }
}

impl<N: Node, V: Clone + PartialEq> Contain<N> for UndirectedGraph<N, V> {
    fn contains(&self, elem: &N) -> bool {
        self.store.contains_key(elem)
    }
}

impl<N: Node, V: Clone + PartialEq> Contain<Edge<N, V>> for UndirectedGraph<N, V> {
    fn contains(&self, elem: &Edge<N, V>) -> bool {
        self.edges().contains(elem)
    }
}

impl<N: Node, V: Clone + PartialEq> Graph<N, V> for UndirectedGraph<N, V> {
    fn new() -> Self {
        Self {
            store: AdjacencyTable::new(),
        }
    }
    fn add_edge(&mut self, edge: Edge<N, V>) {
        let pair = edge.pair();
        self.add_node(pair.0.clone());
        self.add_node(pair.1.clone());

        self.store.entry(pair.0.clone()).and_modify(|e| {
            e.push((pair.1.clone(), edge.value()));
        });
        self.store.entry(pair.1).and_modify(|e| {
            e.push((pair.0, edge.value()));
        });
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

impl<N: Node, V: Clone + PartialEq> Subgraph<N, V> for UndirectedGraph<N, V> {}

impl<N: Node, V: Clone + PartialEq> From<AdjacencyTable<N, V>> for UndirectedGraph<N, V> {
    fn from(adjacency_table: AdjacencyTable<N, V>) -> Self {
        Self {
            store: adjacency_table,
        }
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

        assert_eq!(graph.neighbors("a").unwrap(), &vec![("b", 5), ("c", 7)]);
    }
}
