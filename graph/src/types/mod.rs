/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::kinds::*;

pub mod edge;
pub mod kinds;
pub mod node;

pub(crate) mod prelude {
    pub use crate::kinds::*;
}