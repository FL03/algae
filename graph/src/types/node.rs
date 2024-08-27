/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::marker::PhantomData;

pub struct Node<T, K> {
    pub data: Option<T>,
    _class: PhantomData<K>,
}