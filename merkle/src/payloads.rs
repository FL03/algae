/*
    Appellation: nodes <merkle>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Node;
use decanter::prelude::Hashable;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Hashable, Serialize))]
pub enum Payload<T = String>
where
    T: Hashable,
{
    Leaf(T),
    Node(Box<Node<T>>, Box<Node<T>>),
}

impl<T> Payload<T>
where
    T: Hashable,
{
    pub fn leaf(data: T) -> Self {
        Self::Leaf(data)
    }
    pub fn node(left: Box<Node<T>>, right: Box<Node<T>>) -> Self {
        Self::Node(left, right)
    }
    pub fn is_leaf(&self) -> bool {
        match self {
            Self::Leaf(_) => true,
            _ => false,
        }
    }
    pub fn is_node(&self) -> bool {
        match self {
            Self::Node(_, _) => true,
            _ => false,
        }
    }
}

impl<T> From<T> for Payload<T>
where
    T: Hashable,
{
    fn from(data: T) -> Self {
        Self::Leaf(data)
    }
}

impl<T> From<(Box<Node<T>>, Box<Node<T>>)> for Payload<T>
where
    T: Hashable,
{
    fn from(data: (Box<Node<T>>, Box<Node<T>>)) -> Self {
        Self::node(data.0, data.1)
    }
}
