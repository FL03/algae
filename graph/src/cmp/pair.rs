/*
    Appellation: pair <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: a pair can either be scalar or vector; if vector, than direction matters
*/
use crate::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Pair<T>(pub T, pub T)
where
    T: Node;

impl<T> Pair<T>
where
    T: Node,
{
    pub fn new(a: T, b: T) -> Self {
        Self(a, b)
    }
    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.0, &mut self.1)
    }
    pub fn reverse_mut(&mut self) -> &mut Self {
        self.reverse();
        self
    }
    pub fn pair(&self) -> (T, T) {
        (self.0.clone(), self.1.clone())
    }
}

impl<T> From<(T, T)> for Pair<T>
where
    T: Node,
{
    fn from(data: (T, T)) -> Self {
        Self(data.0, data.1)
    }
}
