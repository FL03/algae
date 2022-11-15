/*
    Appellation: specs <btree>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{BTreeProps, Node};
use std::fmt::Debug;

pub trait BaseObj: Copy + Default + Debug + Ord {}

pub trait BTreeSpec<T: BaseObj> {
    fn root(&self) -> &Node<T>;
    fn root_mut(&mut self) -> &mut Node<T>;
    fn props(&self) -> &BTreeProps;
    fn add_child(&mut self, root: Node<T>) -> &Self {
        self.root_mut().children.insert(0, root);
        self
    }
}

pub trait BTreeSpecExt<T: BaseObj>: BTreeSpec<T> {
    fn swap_root(&mut self, with: &mut Node<T>) {
        std::mem::swap(with, self.root_mut())
    }
    fn insert(&mut self, key: T) {
        if self.props().is_maxed_out(self.root()) {
            // Create an empty root and split the old root...
            let mut new_root = Node::from(self.props().clone().degree);
            self.swap_root(&mut new_root);
            self.add_child(new_root);
            self.props().clone().split_child(self.root_mut(), 0);
        }
        self.props().clone().insert_non_full(self.root_mut(), key);
    }

    fn traverse(&self) {
        BTreeProps::traverse_node(self.root(), 0);
        println!();
    }

    fn search(&self, key: T) -> bool {
        let mut current_node = self.root();
        let mut index: isize;
        loop {
            index = isize::try_from(current_node.keys.len()).ok().unwrap() - 1;
            while index >= 0 && current_node.keys[index as usize] > key {
                index -= 1;
            }

            let u_index: usize = usize::try_from(index + 1).ok().unwrap();
            if index >= 0 && current_node.keys[u_index - 1] == key {
                break true;
            } else if current_node.is_leaf() {
                break false;
            } else {
                current_node = &current_node.children[u_index];
            }
        }
    }
}
