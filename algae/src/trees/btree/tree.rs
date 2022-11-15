/*
    Appellation: btree <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::trees::btree::{BTreeProps, BTreeSpec, BTreeSpecExt, BaseObj, Node};
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BTree<T> {
    pub root: Node<T>,
    pub props: BTreeProps,
}

impl<T> BTree<T>
where
    T: BaseObj,
{
    pub fn new(root: Node<T>, props: BTreeProps) -> Self {
        Self { root, props }
    }
}

impl<T> BTreeSpec<T> for BTree<T>
where
    T: BaseObj,
{
    fn root(&self) -> &Node<T> {
        &self.root
    }
    fn root_mut(&mut self) -> &mut Node<T> {
        &mut self.root
    }
    fn props(&self) -> &BTreeProps {
        &self.props
    }
}

impl<T> BTreeSpecExt<T> for BTree<T> where T: BaseObj {}

/// Create a new tree given the branch factor
impl<T> From<usize> for BTree<T>
where
    T: BaseObj,
{
    fn from(data: usize) -> Self {
        let degree = 2 * data;
        BTree::new(Node::from(degree), BTreeProps::new(degree))
    }
}

#[cfg(test)]
mod tests {
    use super::BTree;
    use crate::trees::btree::{BTreeSpecExt, Data};

    #[test]
    fn test_search() {
        let data = [10, 20, 30, 5, 6, 7, 11, 12, 15]
            .iter()
            .map(|i| Data::new(*i))
            .collect::<Vec<Data<isize>>>();
        let mut tree = BTree::from(2); // Creates a new tree with a branch factor of 2 (degree of 4)
        for i in data {
            tree.insert(i);
        }
        assert!(tree.search(Data::new(15)));
        assert!(!tree.search(Data::new(16)));
    }
}
