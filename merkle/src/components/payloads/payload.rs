/*
    Appellation: nodes <merkle>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{Leaf, Node};
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Payload<T: ToString> {
    Leaf(Leaf<T>),
    Node(Box<Node<T>>, Box<Node<T>>),
}

impl<T: ToString> std::fmt::Display for Payload<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
