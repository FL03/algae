/*
    Appellation: merkle <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Implements the expected leaf element for composing merkle trees
*/
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Data<T>(T);

impl<T> Data<T> {
    pub fn new(data: T) -> Self {
        Data(data)
    }
    pub fn data(&self) -> &T {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Leaf<T: ToString>(Data<T>);

impl<T: ToString> Leaf<T> {
    pub fn new(data: T) -> Self {
        Self(Data(data))
    }
}
