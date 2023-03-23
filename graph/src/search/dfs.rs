/*
    Appellation: dfs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Node;
use crate::{Contain, Graph};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DepthFirstSearch<N: Node> {
    stack: Vec<N>,
    visited: Vec<N>,
}

impl<N: Node> DepthFirstSearch<N> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            visited: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.stack.clear();
        self.visited.clear();
    }
    pub fn search<V>(&mut self, graph: impl Graph<N, V>, start: N) -> Vec<N>
    where
        V: Clone + PartialEq,
    {
        self.stack.push(start);
        while let Some(node) = self.stack.pop() {
            if !self.visited.contains(&node) {
                self.visited.push(node.clone());
                if let Ok(neighbor) = graph.neighbors(node) {
                    for (node, _) in neighbor {
                        self.stack.push(node.clone());
                    }
                }
            }
        }
        self.visited.clone()
    }
}

impl<N: Node> Contain<N> for DepthFirstSearch<N> {
    fn contains(&self, elem: &N) -> bool {
        self.visited.contains(elem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmp::Edge;
    use crate::directed::DirectedGraph;

    const TEST_EDGES: [(&str, &str, usize); 5] = [
        ("a", "b", 5),
        ("c", "a", 7),
        ("b", "c", 10),
        ("d", "c", 10),
        ("e", "f", 10),
    ];

    #[test]
    fn test_dfs() {
        let mut graph = DirectedGraph::<&str, usize>::new();
        for i in TEST_EDGES {
            graph.add_edge(Edge::from(i));
        }
        //
        let mut dfs = DepthFirstSearch::new();
        //
        dfs.search(graph, "a");
        //
        assert!(dfs.contains_all(["b", "c", "a"]));
    }
}
