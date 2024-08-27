/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, strum::EnumIs, thiserror::Error)]
pub enum GraphError {
    
    #[error("[CycleError] {0}")]
    CycleError(String),
    #[error("[UnknownError] {0}")]
    Unknown(String)
}