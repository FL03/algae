/*
    Appellation: nodes <merkle>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Node;
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Payload<T: ToString> {
    Leaf(T),
    Node(Box<Node<T>>, Box<Node<T>>),
}

impl<T> Payload<T> where T: ToString {
    pub fn new_leaf(data: T) -> Self {
        Self::Leaf(data)
    }
    pub fn new_node(left: Box<Node<T>>, right: Box<Node<T>>) -> Self {
        Self::Node(left, right)
    }
}


impl<T> std::fmt::Display for Payload<T> where T: Serialize + ToString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl<T: Clone + ToString> std::convert::From<T> for Payload<T> {
    fn from(data: T) -> Self {
        Self::Leaf(data)
    }
}

impl<T> std::convert::From<(Box<Node<T>>, Box<Node<T>>)> for Payload<T> where T: ToString {
    fn from(data: (Box<Node<T>>, Box<Node<T>>)) -> Self {
        Self::new_node(data.0, data.1)
    }
}
