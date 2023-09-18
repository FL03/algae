/*
    Appellation: linked <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{iter::*, singly::SinglyLinkedList, utils::*};

pub(crate) mod iter;

pub mod singly;

pub(crate) mod utils {}

#[cfg(test)]
mod tests {}
