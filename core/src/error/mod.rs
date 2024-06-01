/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{err::*, kinds::*};

pub(crate) mod err;
pub(crate) mod kinds;

pub type Result<T = ()> = core::result::Result<T, Error>;

pub trait ErrorKind {}

impl<T> ErrorKind for T where T: core::fmt::Display {}