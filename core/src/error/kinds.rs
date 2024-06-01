/*
    Appellation: kinds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/


#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumMessage,
    strum::VariantNames,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(rename_all = "PascalCase", tag = "kind"))]
#[strum(serialize_all = "PascalCase")]
pub enum Errors<T = String> {
    Custom(T),
    External(T),
    #[default]
    #[strum(message = "An unknown error occurred.")]
    Unknown
}

impl<T> core::fmt::Display for Errors<T> where T: core::fmt::Display {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Custom(e) => write!(f, "{}", e),
            Self::External(e) => write!(f, "{}", e),
            Self::Unknown => write!(f, "An unknown error occurred."),
        }
    }
}