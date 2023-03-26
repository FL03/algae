/*
    Appellation: store <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
pub use self::{matrix::*, table::*};

pub(crate) mod matrix;
pub(crate) mod table;

use crate::{cmp::Edge, Contain, Node, Weight};
use serde::{Deserialize, Serialize};
use std::ops::IndexMut;

pub struct Entry<N, V>
where
    N: Node,
    V: Weight,
{
    key: N,
    value: Vec<(N, V)>,
}

impl<N, V> Entry<N, V>
where
    N: Node,
    V: Weight,
{
    pub fn new(key: N, value: Vec<(N, V)>) -> Self {
        Self { key, value }
    }
    pub fn key(&self) -> &N {
        &self.key
    }
    pub fn value(&self) -> &Vec<(N, V)> {
        &self.value
    }
    pub fn value_mut(&mut self) -> &mut Vec<(N, V)> {
        &mut self.value
    }
}

impl<N, V> Contain<(N, V)> for Entry<N, V>
where
    N: Node,
    V: Weight,
{
    fn contains(&self, elem: &(N, V)) -> bool {
        self.value.contains(elem)
    }
}

pub trait Store<N, V>: Extend<Edge<N, V>> + IndexMut<N, Output = Vec<(N, V)>>
where
    N: Node,
    V: Weight,
{
    fn clear(&mut self);
    fn contains_key(&self, key: &N) -> bool;
    fn drain(&mut self);
    fn entry(&mut self, key: N);
    fn get(&self, key: &N) -> Option<&Vec<(N, V)>> {
        if self.contains_key(key) {
            Some(&self[key.clone()])
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Stores<N, V>
where
    N: Node,
    V: Weight,
{
    Matrix(AdjacencyMatrix<N, V>),
    Table(AdjacencyTable<N, V>),
}
