/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, statics::*, types::*};

mod constants {}

mod statics {}

mod types {
    use decanter::prelude::H256;
    use std::collections::HashMap;

    pub type RangeMap<T> = HashMap<H256, T>;
}
