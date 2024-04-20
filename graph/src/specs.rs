/*
    Appellation: specs <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Contain<T>
where
    T: PartialEq,
{
    /// [contains](Contain::contains) returns true if the given element is in the collection
    fn contains(&self, elem: &T) -> bool;
    /// [all](Contain::all) returns true if all elements in the iterator are in the collection
    fn all(&self, iter: impl IntoIterator<Item = T>) -> bool {
        iter.into_iter().all(|i| self.contains(&i))
    }
    /// [any](Contain::any) returns true if *any* element in the given iterator is in the [Contain] instance
    fn any(&self, iter: impl IntoIterator<Item = T>) -> bool {
        iter.into_iter().any(|i| self.contains(&i))
    }
}

/// [Node] describes compatible vertices of the [crate::Graph]
pub trait Node: Clone + Default + Eq + core::hash::Hash {}

impl<T> Node for T where T: Clone + Default + Eq + core::hash::Hash {}

pub trait Weight: Clone + PartialEq {}

impl<T> Weight for T where T: Clone + PartialEq {}
