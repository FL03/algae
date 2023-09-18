/*
    Appellation: singly <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
// use super::SinglyLinkedList;
use super::singly::{Node, SinglyLinkedList};

pub struct IntoIter<T>(SinglyLinkedList<T>);

impl<T> IntoIter<T> {
    pub fn new(list: SinglyLinkedList<T>) -> Self {
        Self(list)
    }
}

pub struct Iter<'a, T> {
    pub(crate) next: Option<&'a Node<T>>,
}

impl<'a, T> Iter<'a, T> {
    pub unsafe fn new(list: &'a SinglyLinkedList<T>) -> Self {
        Self {
            next: list.head().as_ref(),
        }
    }
}

pub struct IterMut<'a, T> {
    pub(crate) next: Option<&'a mut Node<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.link().as_ref();
                node.elem()
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.link().as_mut();
                node.elem_mut()
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iter() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
