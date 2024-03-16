/*
    Appellation: atable <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Node, Weight};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::{hash_map, HashMap};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AdjacencyTable<N, V>
where
    N: Node,
    V: Weight,
{
    store: HashMap<N, Vec<(N, V)>>,
}

impl<N, V> AdjacencyTable<N, V>
where
    N: Node,
    V: Weight,
{
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
    pub fn capacity(&self) -> usize {
        self.store.capacity()
    }
    pub fn clear(&mut self) {
        self.store.clear()
    }
    pub fn contains_key(&self, key: &N) -> bool {
        self.store.contains_key(key)
    }
    pub fn drain(&mut self) -> hash_map::Drain<'_, N, Vec<(N, V)>> {
        self.store.drain()
    }
    pub fn entry(&mut self, key: N) -> hash_map::Entry<'_, N, Vec<(N, V)>> {
        self.store.entry(key)
    }
    pub fn insert(&mut self, key: N, val: Vec<(N, V)>) -> Option<Vec<(N, V)>> {
        self.store.insert(key, val)
    }
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
    pub fn iter(&self) -> hash_map::Iter<'_, N, Vec<(N, V)>> {
        self.store.iter()
    }
    pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, N, Vec<(N, V)>> {
        self.store.iter_mut()
    }
    pub fn get(&self, key: &N) -> Option<&Vec<(N, V)>> {
        self.store.get(key)
    }
    pub fn get_key_value(&self, key: &N) -> Option<(&N, &Vec<(N, V)>)> {
        self.store.get_key_value(key)
    }
    pub fn get_mut(&mut self, key: &N) -> Option<&mut Vec<(N, V)>> {
        self.store.get_mut(key)
    }
    pub fn keys(&self) -> hash_map::Keys<N, Vec<(N, V)>> {
        self.store.keys()
    }
    pub fn len(&self) -> usize {
        self.store.len()
    }
    pub fn remove(&mut self, key: &N) -> Option<Vec<(N, V)>> {
        self.store.remove(key)
    }
    pub fn reserve(&mut self, additional: usize) {
        self.store.reserve(additional)
    }
    pub fn table(self) -> HashMap<N, Vec<(N, V)>> {
        self.store
    }
    pub fn values(&self) -> hash_map::Values<N, Vec<(N, V)>> {
        self.store.values()
    }
    pub fn values_mut(&mut self) -> hash_map::ValuesMut<N, Vec<(N, V)>> {
        self.store.values_mut()
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            store: HashMap::with_capacity(capacity),
        }
    }
}

impl<N, V> Extend<(N, Vec<(N, V)>)> for AdjacencyTable<N, V>
where
    N: Node,
    V: Weight,
{
    fn extend<T: IntoIterator<Item = (N, Vec<(N, V)>)>>(&mut self, iter: T) {
        self.store.extend(iter)
    }
}

impl<N, V> From<HashMap<N, Vec<(N, V)>>> for AdjacencyTable<N, V>
where
    N: Node,
    V: Weight,
{
    fn from(store: HashMap<N, Vec<(N, V)>>) -> Self {
        Self { store }
    }
}

impl<N, V> FromIterator<(N, Vec<(N, V)>)> for AdjacencyTable<N, V>
where
    N: Node,
    V: Weight,
{
    fn from_iter<T: IntoIterator<Item = (N, Vec<(N, V)>)>>(iter: T) -> Self {
        let mut map = HashMap::with_hasher(Default::default());
        map.extend(iter);
        AdjacencyTable::from(map)
    }
}

impl<N, V> IntoIterator for AdjacencyTable<N, V>
where
    N: Node,
    V: Weight,
{
    type Item = (N, Vec<(N, V)>);

    type IntoIter = hash_map::IntoIter<N, Vec<(N, V)>>;

    fn into_iter(self) -> Self::IntoIter {
        self.store.into_iter()
    }
}

impl<N, V> std::ops::Index<N> for AdjacencyTable<N, V>
where
    N: Node,
    V: Weight,
{
    type Output = Vec<(N, V)>;

    fn index(&self, index: N) -> &Self::Output {
        &self.store[&index]
    }
}

impl<N, V> std::ops::IndexMut<N> for AdjacencyTable<N, V>
where
    N: Node,
    V: Weight,
{
    fn index_mut(&mut self, index: N) -> &mut Self::Output {
        self.store.get_mut(&index).unwrap()
    }
}
