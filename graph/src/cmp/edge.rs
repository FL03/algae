/*
    Appellation: edge <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: an edge consists of two nodes and an optional edge value
*/
use super::{Node, Pair};
use serde::{Deserialize, Serialize};

pub trait Related<N: Node, V> {}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Edge<N: Node, V: Clone>(N, N, V);

impl<N: Node, V: Clone> Edge<N, V> {
    pub fn new(a: N, b: N, v: V) -> Self {
        Self(a, b, v)
    }
    pub fn pair(&self) -> Pair<N> {
        Pair::new(self.0.clone(), self.1.clone())
    }
    pub fn value(&self) -> V {
        self.2.clone()
    }
}

impl<N: Node, V: Clone> From<(N, N, V)> for Edge<N, V> {
    fn from(data: (N, N, V)) -> Self {
        Self(data.0, data.1, data.2)
    }
}
