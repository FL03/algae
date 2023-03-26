/*
    Appellation: search <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{bfs::BreadthFirstSearch, dfs::DepthFirstSearch};

mod bfs;
mod dfs;

use crate::{Contain, Graph, Node, Weight};

/// [Searcher] is a trait that defines the behavior of a graph search algorithm.
pub trait Searcher<N, V>: Contain<N>
where
    N: Node,
    V: Weight,
{
    /// Search the graph for nodes.
    fn search(&mut self, graph: impl Graph<N, V>, start: N) -> Vec<N>;
    /// Reset the search state.
    fn reset(&mut self);
}
