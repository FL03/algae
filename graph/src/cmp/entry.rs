/*
    Appellation: entry <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Entry
//!
//!
use crate::prelude::Contain;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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

impl<N, V> Contain<N> for Entry<N, V>
where
    N: PartialEq,
{
    fn contains(&self, node: &N) -> bool {
        self.value.iter().any(|(n, _)| n == node)
    }
}
