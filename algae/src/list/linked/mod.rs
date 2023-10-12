/*
    Appellation: linked <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{singly::SinglyLinkedList, utils::*};

pub mod singly;
pub mod stack;

pub(crate) mod utils {}

#[cfg(test)]
mod tests {}
