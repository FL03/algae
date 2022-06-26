/*
   Appellation: trees
   Context: module
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
*/
pub(crate) mod avtree;
pub(crate) mod btree;
pub(crate) mod merkle;
pub(crate) mod rbtree;

pub use crate::data::structures::trees::{avtree::*, btree::*, merkle::*, rbtree::*};
