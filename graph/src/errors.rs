/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::cycle::*;

pub(crate) mod cycle;

use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    Clone,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SmartDefault,
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[strum(serialize_all = "snake_case")]
pub enum GraphError {
    Cycle(CycleError),
    NodeInGraph,
    #[default]
    NodeNotInGraph,
    Unknown(String),
}

#[cfg(feature = "std")]
impl std::error::Error for GraphError {}

macro_rules! impl_from_err {
    ($($variant:ident<$err:ident>),*) => {
        $(
            impl_from_err!(@impl $variant<$err>);
        )*
    };
    (@impl $variant:ident<$err:ident>) => {
        impl From<$err> for GraphError {
            fn from(err: $err) -> Self {
                Self::$variant(err)
            }
        }
    };
}

impl_from_err!(Cycle<CycleError>);
