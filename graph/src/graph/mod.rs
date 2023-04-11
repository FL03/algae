/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements an abstract graph data structure
*/
use crate::store::Entry;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

pub trait GraphStore<N, V>: Default {
    fn capacity(&self) -> usize;
    fn clear(&mut self);
    fn dim(&self) -> (usize, usize);
    fn get_entries(&self, key: &N) -> Option<&Entry<N, V>>;
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
pub enum GraphType {
    Directed,
    #[default]
    Undirected,
}
