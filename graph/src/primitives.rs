/*
   Appellation: primitives <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description:
*/
pub use self::{constants::*, types::*};

pub(crate) mod constants {}

pub(crate) mod types {
    use std::collections::HashMap;

    pub type AdjacencyHashTable<T = String> = HashMap<T, Vec<(T, i32)>>;
}
