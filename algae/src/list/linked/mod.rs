/*
    Appellation: linked <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::singly::SinglyLinkedList;

pub mod singly;
pub mod stack;

pub(crate) mod prelude {
    pub use super::singly::prelude::*;
    pub use super::stack::Stack;
}

#[cfg(test)]
mod tests {}
