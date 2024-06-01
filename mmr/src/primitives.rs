/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::types::*;

pub mod types {
    use decanter::prelude::H256;
    use std::collections::HashMap;

    pub type RangeMap<T> = HashMap<H256, T>;
}
