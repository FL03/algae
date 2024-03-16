/*
    Appellation: store <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Store
pub use self::{matrix::*, table::*};

mod matrix;
mod table;

use crate::{Edge, Node, Weight};

use std::ops::IndexMut;

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
