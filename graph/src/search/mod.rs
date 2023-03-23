/*
    Appellation: search <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub mod bfs;
pub mod dfs;

use crate::{cmp::Node, Contain, Graph};

pub trait BreadthFirstSearchable<N: Node, V: Clone + PartialEq> {
    fn breadth_first_search(&self, start: N) -> Vec<N>;
}

pub trait Searcher<N: Node, V: Clone + PartialEq>: Contain<N> {
    fn search(&mut self, graph: impl Graph<N, V>, start: N) -> Vec<N>;
}
