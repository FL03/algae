/*
    Appellation: algo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
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
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
#[strum(serialize_all = "snake_case")]
pub enum CycleError {
    #[default]
    CycleDetected,
}

#[cfg(feature = "std")]
impl std::error::Error for CycleError {}
