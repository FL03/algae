/*
    Appellation: atable <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: an adjacency table
*/
use super::Node;
use serde::{Deserialize, Serialize};
use std::collections::{hash_map, HashMap};
use std::iter::Extend;

pub trait HashMapLike<K: Eq + std::hash::Hash, V>:
    Extend<(K, V)>
    + FromIterator<(K, V)>
    + IntoIterator<Item = (K, V), IntoIter = hash_map::IntoIter<K, V>>
{
    fn new() -> Self;
    fn capacity(&self) -> usize;
    fn clear(&mut self) {
        self.table_mut().clear()
    }
    fn drain(&mut self) -> hash_map::Drain<'_, K, V>;
    fn entry(&mut self, key: K) -> hash_map::Entry<'_, K, V> {
        self.table_mut().entry(key)
    }
    fn insert(&mut self, key: K, val: V) -> Option<V>;
    fn get(&self, key: &K) -> Option<&V>;
    fn keys(&self) -> hash_map::Keys<K, V> {
        self.table().keys()
    }
    fn table(&self) -> &HashMap<K, V>;
    fn table_mut(&mut self) -> &mut HashMap<K, V>;
}

pub struct KeyValue<K, V>(Vec<(K, Vec<V>)>);

impl<K, V> KeyValue<K, V> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdjacencyTable<N: Node, V>(HashMap<N, Vec<(N, V)>>);

impl<N: Node, V> AdjacencyTable<N, V> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    pub fn clear(&mut self) {
        self.0.clear()
    }
    pub fn contains_key(&self, key: &N) -> bool {
        self.0.contains_key(key)
    }
    pub fn drain(&mut self) -> hash_map::Drain<'_, N, Vec<(N, V)>> {
        self.0.drain()
    }
    pub fn entry(&mut self, key: N) -> hash_map::Entry<'_, N, Vec<(N, V)>> {
        self.0.entry(key)
    }
    pub fn insert(&mut self, key: N, val: Vec<(N, V)>) -> Option<Vec<(N, V)>> {
        self.0.insert(key, val)
    }
    pub fn get(&self, key: &N) -> Option<&Vec<(N, V)>> {
        self.0.get(key)
    }
    pub fn get_key_value(&self, key: &N) -> Option<(&N, &Vec<(N, V)>)> {
        self.0.get_key_value(key)
    }
    pub fn get_mut(&mut self, key: &N) -> Option<&mut Vec<(N, V)>> {
        self.0.get_mut(key)
    }
    pub fn keys(&self) -> hash_map::Keys<N, Vec<(N, V)>> {
        self.0.keys()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn table(self) -> HashMap<N, Vec<(N, V)>> {
        self.0
    }
    pub fn values(&self) -> hash_map::Values<N, Vec<(N, V)>> {
        self.0.values()
    }
    pub fn values_mut(&mut self) -> hash_map::ValuesMut<N, Vec<(N, V)>> {
        self.0.values_mut()
    }
}

impl<N: Node, V> Extend<(N, Vec<(N, V)>)> for AdjacencyTable<N, V> {
    fn extend<T: IntoIterator<Item = (N, Vec<(N, V)>)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl<N: Node, V> FromIterator<(N, Vec<(N, V)>)> for AdjacencyTable<N, V> {
    fn from_iter<T: IntoIterator<Item = (N, Vec<(N, V)>)>>(iter: T) -> Self {
        let mut map = HashMap::with_hasher(Default::default());
        map.extend(iter);
        AdjacencyTable(map)
    }
}

impl<N: Node, V> IntoIterator for AdjacencyTable<N, V> {
    type Item = (N, Vec<(N, V)>);

    type IntoIter = hash_map::IntoIter<N, Vec<(N, V)>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}