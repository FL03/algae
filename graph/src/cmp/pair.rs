/*
    Appellation: pair <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: a pair can either be scalar or vector; if vector, than direction matters
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Pair<T>(pub T, pub T)
where
    T: Default;

impl<T> Pair<T>
where
    T: Default,
{
    pub fn new(a: T, b: T) -> Self {
        Self(a, b)
    }
}

impl<T> From<(T, T)> for Pair<T>
where
    T: Default,
{
    fn from(data: (T, T)) -> Self {
        Self(data.0, data.1)
    }
}
