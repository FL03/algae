/*
    Appellation: singly <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{iter::*, list::*};

pub(crate) mod iter;
pub(crate) mod list;

pub(crate) mod prelude {
    pub use super::list::SinglyLinkedList;
}

#[cfg(test)]
mod tests {}
