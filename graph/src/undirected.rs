/*
    Appellation: undirected <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{cmp::Edge, store::AdjacencyTable};
use crate::{Contain, Graph, GraphExt, Node, Subgraph, Weight};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UndirectedGraph<N = String, V = i64>
where
    N: Node,
    V: Weight,
{
    store: AdjacencyTable<N, V>,
}

impl<N, V> AsMut<AdjacencyTable<N, V>> for UndirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn as_mut(&mut self) -> &mut AdjacencyTable<N, V> {
        &mut self.store
    }
}

impl<N, V> AsRef<AdjacencyTable<N, V>> for UndirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn as_ref(&self) -> &AdjacencyTable<N, V> {
        &self.store
    }
}

impl<N, V> Contain<N> for UndirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn contains(&self, elem: &N) -> bool {
        self.store.contains_key(elem)
    }
}

impl<N, V> Contain<Edge<N, V>> for UndirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn contains(&self, elem: &Edge<N, V>) -> bool {
        self.edges().contains(elem)
    }
}

impl<N, V> Graph<N, V> for UndirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn add_edge(&mut self, edge: Edge<N, V>) {
        let pair = edge.pair();
        self.add_node(pair.0.clone());
        self.add_node(pair.1.clone());

        self.store.entry(pair.0.clone()).and_modify(|e| {
            e.push((pair.1.clone(), edge.value().clone()));
        });
        self.store.entry(pair.1).and_modify(|e| {
            e.push((pair.0, edge.value().clone()));
        });
    }
    fn store_mut(&mut self) -> &mut AdjacencyTable<N, V> {
        &mut self.store
    }
    fn store(&self) -> &AdjacencyTable<N, V> {
        &self.store
    }
}

impl<N, V> GraphExt<N, V> for UndirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn new() -> Self {
        Self {
            store: AdjacencyTable::new(),
        }
    }
    fn with_capacity(capacity: usize) -> Self {
        Self {
            store: AdjacencyTable::with_capacity(capacity),
        }
    }
}

impl<N, V> Subgraph<N, V> for UndirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
}

impl<N, V> From<AdjacencyTable<N, V>> for UndirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn from(store: AdjacencyTable<N, V>) -> Self {
        Self { store }
    }
}

impl<N, V> std::ops::Index<N> for UndirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    type Output = Vec<(N, V)>;

    fn index(&self, index: N) -> &Self::Output {
        &self.store[index]
    }
}

impl<N, V> std::ops::IndexMut<N> for UndirectedGraph<N, V>
where
    N: Node,
    V: Weight,
{
    fn index_mut(&mut self, index: N) -> &mut Self::Output {
        self.store.index_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // assert that the graph contains all the edges
        assert!(graph.contains_all(EXPECTED.into_iter().map(Edge::from).collect::<Vec<_>>()));
        // assert that the graph can be indexed
        assert_eq!(graph["a"], vec![("b", 5), ("c", 7)]);
    }

    #[test]
    fn test_neighbours() {
        let mut graph = UndirectedGraph::new();

        for i in TEST_EDGES {
            graph.add_edge(i.into());
        }

        assert_eq!(graph["a"], vec![("b", 5), ("c", 7)]);
    }
}
