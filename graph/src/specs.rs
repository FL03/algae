/*
    Appellation: specs <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub trait Contain<T> {
    /// [Contain::contains] returns true if the given element is in the [Contain] instance
    fn contains(&self, elem: &T) -> bool;
    /// [Contain::contains_many] returns true if all elements in the given iterator are in the [Contain] instance
    fn contains_many(&self, iter: impl IntoIterator<Item = T>) -> bool {
        iter.into_iter().all(|i| self.contains(&i))
    }
    /// [Contain::contains_some] returns true if any element in the given iterator is in the [Contain] instance
    fn contains_some(&self, iter: impl IntoIterator<Item = T>) -> bool {
        iter.into_iter().any(|i| self.contains(&i))
    }
}

/// [Node] describes compatible vertices of the [crate::Graph]
pub trait Node: Clone + Eq + std::hash::Hash {}

impl Node for char {}

impl Node for &str {}

impl Node for String {}
