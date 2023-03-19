/*
    Appellation: atable <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: an adjacency table
*/
use super::Node;
use ndarray::{Array2, Dim, Shape};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::iter::Extend;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdjacencyMatrix<N: Node> {
    matrix: Array2<usize>,
    nodes: HashSet<N>,
}

impl<N: Node> AdjacencyMatrix<N> {
    pub fn new(capacity: usize) -> Self {
        Self {
            matrix: Array2::zeros((capacity, capacity)),
            nodes: HashSet::with_capacity(capacity),
        }
    }
    pub fn insert(&mut self, elem: N) -> bool {
        self.nodes.insert(elem)
    }
}
