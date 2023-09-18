/*
    Appellation: singly <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{iter::*, store::*, utils::*};

pub(crate) mod iter;
pub(crate) mod store;

pub(crate) mod utils {}

#[cfg(test)]
mod tests {}
