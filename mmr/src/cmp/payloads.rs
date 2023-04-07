/*
    Appellation: rangemap <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::prelude::{Hashable, H256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hashable, PartialEq, Serialize)]
pub struct Payload<T = String>
where
    T: ToString,
{
    id: H256,
    data: HashMap<H256, T>,
}

impl<T> Payload<T>
where
    T: ToString,
{
    pub fn new() -> Self {
        Self {
            id: H256::generate(),
            data: HashMap::new(),
        }
    }
    pub fn data(&self) -> &HashMap<H256, T> {
        &self.data
    }
}

impl<T> From<HashMap<H256, T>> for Payload<T>
where
    T: ToString,
{
    fn from(data: HashMap<H256, T>) -> Self {
        Self {
            id: H256::generate(),
            data,
        }
    }
}

impl<T> std::fmt::Display for Payload<T>
where
    T: ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
