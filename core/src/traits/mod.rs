/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::prelude::*;

pub mod store;

pub(crate) mod prelude {
    pub use super::store::*;
}
