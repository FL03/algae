/*
    Appellation: stores <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::store::*;

mod store;

pub trait Store<T> {}
