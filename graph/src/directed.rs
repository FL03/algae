/*
    Appellation: directed <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{store::AdjacencyTable, Edge, Node, Weight};
use crate::{Contain, Graph, GraphExt, Subgraph};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DirectedGraph<N = String, V = i64>
where
    N: Node,
    V: Weight,
{
    store: AdjacencyTable<N, V>,
}

impl<N, V> DirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    pub fn new() -> Self {
        Self {
            store: AdjacencyTable::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            store: AdjacencyTable::with_capacity(capacity),
        }
    }
}

impl<N, V> AsMut<AdjacencyTable<N, V>> for DirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn as_mut(&mut self) -> &mut AdjacencyTable<N, V> {
        &mut self.store
    }
}

impl<N, V> AsRef<AdjacencyTable<N, V>> for DirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn as_ref(&self) -> &AdjacencyTable<N, V> {
        &self.store
    }
}

impl<N, V> Contain<N> for DirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn contains(&self, elem: &N) -> bool {
        self.store.contains_key(elem)
    }
}

impl<N, V> Contain<Edge<N, V>> for DirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn contains(&self, elem: &Edge<N, V>) -> bool {
        self.edges().contains(elem)
    }
}

impl<N, V> Graph<N, V> for DirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn store_mut(&mut self) -> &mut AdjacencyTable<N, V> {
        &mut self.store
    }
    fn store(&self) -> &AdjacencyTable<N, V> {
        &self.store
    }
}

impl<N, V> GraphExt<N, V> for DirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
}

impl<N, V> Subgraph<N, V> for DirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
}

impl<N, V> From<AdjacencyTable<N, V>> for DirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn from(store: AdjacencyTable<N, V>) -> Self {
        Self { store }
    }
}

impl<N, V> std::ops::Index<N> for DirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    type Output = Vec<(N, V)>;

    fn index(&self, index: N) -> &Self::Output {
        self.store.get(&index).unwrap()
    }
}

impl<N, V> std::ops::IndexMut<N> for DirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn index_mut(&mut self, index: N) -> &mut Self::Output {
        self.store.get_mut(&index).unwrap()
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
        assert!(graph.contains_all(["a", "b", "c"]));
        assert!(graph.contains_some(["a", "b", "c", "d"]));
    }
}
