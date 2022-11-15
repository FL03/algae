/*
   Appellation: primitives <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description:
*/
pub use self::{nullnode::*, types::*};

pub(crate) mod nullnode;

pub(crate) mod types {
    use std::collections::HashMap;

    pub type AdjacencyHashTable<T = String> = HashMap<T, Vec<(T, i32)>>;
}
