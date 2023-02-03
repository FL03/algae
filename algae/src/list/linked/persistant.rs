/*
    Appellation: persistant <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T, next: Link<T>) -> Self {
        Self { elem, next }
    }
    pub fn data(&self) -> &T {
        &self.elem
    }
    pub fn link(&self) -> &Link<T> {
        &self.next
    }
}

/// Singly-Linked, Persistant List
pub struct SLPList<T> {
    head: Link<T>,
}

impl<T> SLPList<T> {
    pub fn new() -> Self {
        Self::from(None)
    }
    pub fn prepend(&self, elem: T) -> Self {
        SLPList::from(Some(Rc::new(Node::new(elem, self.head.clone()))))
    }

    pub fn tail(&self) -> Self {
        SLPList::from(self.head.as_ref().and_then(|node| node.link().clone()))
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| node.data())
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> From<Link<T>> for SLPList<T> {
    fn from(head: Link<T>) -> SLPList<T> {
        SLPList { head }
    }
}

impl<T> Default for SLPList<T> {
    fn default() -> Self {
        Self::from(None)
    }
}

impl<T> Drop for SLPList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linked_list() {
        let list = SLPList::default();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn test_linked_list_iter() {
        let list = SLPList::default().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
