/*
    Appellation: bfs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Searcher;
use crate::prelude::{Contain, Graph, Node, Weight};
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BreadthFirstSearch<N: Node> {
    queue: VecDeque<N>,
    visited: HashSet<N>,
}

impl<N: Node> BreadthFirstSearch<N> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            visited: HashSet::new(),
        }
    }
}

impl<N: Node> Contain<N> for BreadthFirstSearch<N> {
    fn contains(&self, elem: &N) -> bool {
        self.visited.contains(elem)
    }
}

impl<N, V> Searcher<N, V> for BreadthFirstSearch<N>
where
    N: Node,
    V: Weight,
{
    fn reset(&mut self) {
        self.visited.clear();
        self.queue.clear();
    }
    fn search(&mut self, graph: impl Graph<N, V>, start: N) -> Vec<N> {
        self.queue.push_back(start);

        while let Some(node) = self.queue.pop_front() {
            if !self.visited.contains(&node) {
                self.visited.insert(node.clone());

                if let Some(edges) = graph.store().get(&node) {
                    for (n, _) in edges {
                        self.queue.push_back(n.clone());
                    }
                }
            }
        }

        self.visited.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmp::Edge;
    use crate::DirectedGraph;

    const TEST_EDGES: [(&str, &str, usize); 5] = [
        ("a", "b", 5),
        ("c", "a", 7),
        ("b", "c", 10),
        ("d", "c", 10),
        ("e", "f", 10),
    ];

    #[test]
    fn test_bfs_directed() {
        let mut graph = DirectedGraph::<&str, usize>::new();
        for i in TEST_EDGES {
            graph.add_edge(Edge::from(i));
        }
        let mut bfs = BreadthFirstSearch::new();
        bfs.search(graph, "a");
        assert!(bfs.contains_all(["b", "c", "a"]));
    }
}
