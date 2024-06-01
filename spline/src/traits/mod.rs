/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::prelude::*;

pub mod interpolate;

pub(crate) mod prelude {
    pub use super::interpolate::*;
}
