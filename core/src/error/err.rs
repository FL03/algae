/*
    Appellation: err <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Errors, ErrorKind};



#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize),)]
pub struct Error<K = String> where K: ErrorKind {
    kind: Errors<K>,
    message: String,
}

