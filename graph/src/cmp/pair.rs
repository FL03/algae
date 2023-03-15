/*
    Appellation: pair <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: a pair can either be scalar or vector; if vector, than direction matters
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Pair<T: Clone>(pub T, pub T);

impl<T: Clone> Pair<T> {
    pub fn new(a: T, b: T) -> Self {
        Self(a, b)
    }
}
