/*
    Appellation: atable <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Adjacency Table
use crate::Node;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AdjacencyMatrix<N = String, V = i64>
where
    N: Node,
    V: Clone + PartialEq,
{
    store: Vec<(N, Vec<(N, V)>)>,
}

impl<N: Node, V: Clone + PartialEq> AdjacencyMatrix<N, V> {
    pub fn new() -> Self {
        Self { store: Vec::new() }
    }
    /// Clears the matrix, removing all elements.
    pub fn clear(&mut self) {
        self.store.clear()
    }
    /// Returns the number of elements the matrix can hold without reallocating.
    pub fn capacity(&self) -> usize {
        self.store.capacity()
    }
    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, key: &N) -> Option<&Vec<(N, V)>> {
        self.store
            .iter()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }
    /// Returns a reference to the key and its value(s).
    pub fn get_key_value(&self, key: &N) -> Option<(&N, &Vec<(N, V)>)> {
        self.store
            .iter()
            .find_map(|(k, v)| if k == key { Some((k, v)) } else { None })
    }
    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut(&mut self, key: &N) -> Option<&mut Vec<(N, V)>> {
        self.store
            .iter_mut()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }
    /// Returns an iterator over the elements of the matrix.
    pub fn keys(&self) -> impl Iterator<Item = &N> {
        self.store.iter().map(|(k, _)| k)
    }
    /// Returns the number of elements in the matrix.
    pub fn len(&self) -> usize {
        self.store.len()
    }
    /// Pushes a new element to the matrix.
    pub fn push(&mut self, elem: N, value: Vec<(N, V)>) {
        self.store.push((elem, value));
    }
    /// Shrinks the capacity of the matrix as much as possible.
    pub fn shrink_to_fit(&mut self) {
        self.store.shrink_to_fit()
    }
    /// Reserves capacity for at least `additional` more elements to be inserted in the matrix.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            store: Vec::with_capacity(capacity),
        }
    }
}

impl<N: Node, V: Clone + PartialEq> Extend<(N, Vec<(N, V)>)> for AdjacencyMatrix<N, V> {
    fn extend<T: IntoIterator<Item = (N, Vec<(N, V)>)>>(&mut self, iter: T) {
        self.store.extend(iter);
    }
}

impl<N, V> Index<N> for AdjacencyMatrix<N, V>
where
    N: Node,
    V: Clone + PartialEq,
{
    type Output = Vec<(N, V)>;

    fn index(&self, index: N) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

impl<N, V> Index<&N> for AdjacencyMatrix<N, V>
where
    N: Node,
    V: Clone + PartialEq,
{
    type Output = Vec<(N, V)>;

    fn index(&self, index: &N) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<N, V> IndexMut<N> for AdjacencyMatrix<N, V>
where
    N: Node,
    V: Clone + PartialEq,
{
    fn index_mut(&mut self, index: N) -> &mut Self::Output {
        self.get_mut(&index).unwrap()
    }
}
