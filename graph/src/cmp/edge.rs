/*
    Appellation: edge <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: an edge consists of two nodes and an optional edge value
*/
use super::Pair;
use crate::Node;
use serde::{Deserialize, Serialize};

pub trait Related<N: Node, V> {}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Edge<N = String, V = i64>
where
    N: Node,
{
    pair: Pair<N>,
    weight: V,
}

impl<N, V> Edge<N, V>
where
    N: Node,
{
    pub fn new(a: N, b: N, weight: V) -> Self {
        Self {
            pair: Pair::new(a, b),
            weight,
        }
    }
    pub fn pair(&self) -> Pair<N> {
        self.pair.clone()
    }
    pub fn value(&self) -> &V {
        &self.weight
    }
}

impl<N, V> From<(N, N, V)> for Edge<N, V>
where
    N: Node,
{
    fn from(data: (N, N, V)) -> Self {
        Self::new(data.0, data.1, data.2)
    }
}

impl<N, V> From<(Pair<N>, V)> for Edge<N, V>
where
    N: Node,
{
    fn from(data: (Pair<N>, V)) -> Self {
        Self {
            pair: data.0,
            weight: data.1,
        }
    }
}
