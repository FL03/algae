/*
   Appellation: proofs <merkle>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Merkle Proofs
//!
//! ## Overview
//!
//! Merkle proofs are a way to prove that a given piece of data is part of a Merkle tree.
//!
pub use self::{path::*, proof::*, utils::*};

pub(crate) mod path;
pub(crate) mod proof;

use decanter::prelude::H256;

pub trait MerkleProof {
    fn proof(&self) -> Vec<H256>;
}

pub(crate) mod utils {}
