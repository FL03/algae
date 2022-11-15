/*
    Appellation: props <btree>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::trees::btree::{BaseObj, Node};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Debug};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BTreeProps {
    pub degree: usize,
    pub max_keys: usize,
    pub mid_key_index: usize,
}

impl BTreeProps {
    pub fn new(degree: usize) -> Self {
        BTreeProps {
            degree,
            max_keys: degree - 1,
            mid_key_index: (degree - 1) / 2,
        }
    }

    pub fn is_maxed_out<T>(&self, node: &Node<T>) -> bool
    where
        T: BaseObj,
    {
        node.keys.len() == self.max_keys
    }

    pub fn split_child<T>(&self, parent: &mut Node<T>, child_index: usize)
    where
        T: BaseObj,
    {
        let child = parent.fetch_child(child_index);
        let middle_key = child.keys[self.mid_key_index];
        let right_keys = match child.keys.split_off(self.mid_key_index).split_first() {
            Some((_first, _others)) => {
                // We don't need _first, as it will move to parent node.
                _others.to_vec()
            }
            None => Vec::with_capacity(self.max_keys),
        };
        let right_children = if !child.is_leaf() {
            Some(child.children.split_off(self.mid_key_index + 1))
        } else {
            None
        };
        let new_child_node: Node<T> = Node::new(right_children, self.degree, Some(right_keys));

        parent.keys.insert(child_index, middle_key);
        parent.children.insert(child_index + 1, new_child_node);
    }

    pub fn insert_non_full<T>(&mut self, node: &mut Node<T>, key: T)
    where
        T: BaseObj,
    {
        let mut index: isize = isize::try_from(node.keys.len()).ok().unwrap() - 1;
        while index >= 0 && node.keys[index as usize] >= key {
            index -= 1;
        }

        let mut u_index: usize = usize::try_from(index + 1).ok().unwrap();
        if node.is_leaf() {
            // Just insert it, as we know this method will be called only when node is not full
            node.keys.insert(u_index, key);
        } else {
            if self.is_maxed_out(&node.children[u_index]) {
                self.split_child(node, u_index);
                if node.keys[u_index] < key {
                    u_index += 1;
                }
            }

            self.insert_non_full(&mut node.children[u_index], key);
        }
    }
    pub fn traverse_node<T>(node: &Node<T>, depth: usize)
    where
        T: BaseObj,
    {
        if node.is_leaf() {
            print!(" {0:{<1$}{2:?}{0:}<1$} ", "", depth, node.keys);
        } else {
            let _depth = depth + 1;
            for (index, key) in node.keys.iter().enumerate() {
                Self::traverse_node(&node.children[index], _depth);
                print!("{0:{<1$}{2:?}{0:}<1$}", "", depth, key);
            }
            Self::traverse_node(node.children.last().unwrap(), _depth);
        }
    }
}
