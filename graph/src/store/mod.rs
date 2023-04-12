/*
    Appellation: store <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
pub use self::{matrix::*, table::*};

mod matrix;
mod table;

use crate::{Contain, Edge, Node};
use serde::{Deserialize, Serialize};
use std::ops::IndexMut;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Entry<N = String, V = ()> {
    key: N,
    value: Vec<(N, V)>,
}

impl<N, V> Entry<N, V> {
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
    N: PartialEq,
    V: PartialEq,
{
    fn contains(&self, elem: &(N, V)) -> bool {
        self.value.contains(elem)
    }
}

pub trait Store<N, V>: Extend<Edge<N, V>> + IndexMut<N, Output = Vec<(N, V)>>
where
    N: Node,
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
