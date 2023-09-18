/*
    Appellation: singly <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{IntoIter, Iter, IterMut};
use std::ptr;

type Link<T> = *mut Node<T>;

// pub type LinkMut<T> = Option<*mut Node<T>>;

pub struct SinglyLinkedList<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }
    pub fn head(&self) -> Link<T> {
        self.head
    }
    pub fn push(&mut self, elem: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node::new(elem, ptr::null_mut())));

            if self.tail.is_null() {
                self.head = new_tail;
            } else {
                (*self.tail).next = new_tail;
            }
            // Update the tail
            self.tail = new_tail;
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(head.elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.elem) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.elem) }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe { Iter::new(self) }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<T> IntoIterator for SinglyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T, next: Link<T>) -> Self {
        Node { elem, next }
    }

    pub fn elem(&self) -> &T {
        &self.elem
    }

    pub fn elem_mut(&mut self) -> &mut T {
        &mut self.elem
    }

    pub fn link(&self) -> Link<T> {
        self.next
    }

    pub unsafe fn next(&self) -> Option<&Node<T>> {
        self.next.as_ref()
    }

    pub unsafe fn next_mut(&mut self) -> Option<&mut Node<T>> {
        self.next.as_mut()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basics() {
        let mut list = SinglyLinkedList::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_into_iter() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

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
    fn test_iter_mut() {
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

    #[test]
    fn miri_food() {
        let mut list = SinglyLinkedList::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        list.push(4);
        assert_eq!(list.pop(), Some(2));
        list.push(5);

        assert_eq!(list.peek(), Some(&3));
        list.push(6);
        list.peek_mut().map(|x| *x *= 10);
        assert_eq!(list.peek(), Some(&30));
        assert_eq!(list.pop(), Some(30));

        for elem in list.iter_mut() {
            *elem *= 100;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert_eq!(list.pop(), Some(400));
        list.peek_mut().map(|x| *x *= 10);
        assert_eq!(list.peek(), Some(&5000));
        list.push(7);

        // Drop it on the ground and let the dtor exercise itself
    }
}
