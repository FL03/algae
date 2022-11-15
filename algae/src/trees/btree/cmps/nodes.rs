/*
    Appellation: nodes <btree>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Node<T> {
    pub children: Vec<Node<T>>,
    pub keys: Vec<T>,
}

impl<T> Node<T>
where
    T: Ord,
{
    pub fn new(children: Option<Vec<Node<T>>>, degree: usize, keys: Option<Vec<T>>) -> Self {
        let children = children.unwrap_or_else(|| Vec::with_capacity(degree));
        let keys = keys.unwrap_or_else(|| Vec::with_capacity(degree - 1));
        Node { children, keys }
    }

    pub fn children(&self) -> &Vec<Node<T>> {
        &self.children
    }

    pub fn fetch_child(&mut self, index: usize) -> &mut Node<T> {
        &mut self.children[index]
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl<T> From<usize> for Node<T>
where
    T: Ord,
{
    fn from(data: usize) -> Self {
        Self::new(None, data, None)
    }
}
