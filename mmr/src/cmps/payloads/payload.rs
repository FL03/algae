/*
    Appellation: rangemap <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::From};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RangeMap<T: Hashable>(pub HashMap<H256, T>);

impl<T: Hashable> RangeMap<T> {
    pub fn data(&self) -> &HashMap<H256, T> {
        &self.0
    }
}

impl<T: Hashable> From<HashMap<H256, T>> for RangeMap<T> {
    fn from(data: HashMap<H256, T>) -> Self {
        Self(data)
    }
}

impl<T: Hashable + ToString + Serialize> Hashable for RangeMap<T> {
    fn hash(&self) -> H256 {
        hasher(&self).as_slice().to_owned().into()
    }
}

impl<T: Hashable + Serialize> std::fmt::Display for RangeMap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
