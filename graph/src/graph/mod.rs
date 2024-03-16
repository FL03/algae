/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements an abstract graph data structure
*/
use crate::cmp::entry::Entry;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

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
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "lowercase", untagged)
)]
pub enum GraphType {
    Directed,
    #[default]
    Undirected,
}
