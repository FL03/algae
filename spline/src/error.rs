/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub type Result<T = ()> = core::result::Result<T, SplineError>;

macro_rules! error_kind {
    ($(rename_all: $lit:literal,)? $vis:vis enum $name:ident $($rest:tt)*) => {
        error_kind!(@impl $(rename_all: $lit,)? $vis enum $name $($rest)*);
    };
    (@impl $vis:vis enum $name:ident $($rest:tt)*) => {
        error_kind!(@impl rename_all: "PascalCase", $vis enum $name $($rest)*);

    };
    (@impl rename_all: $lit:literal, $vis:vis enum $name:ident $($rest:tt)*) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            strum::AsRefStr,
            strum::Display,
            strum::EnumCount,
            strum::EnumIs,
            strum::EnumIter,
            strum::EnumString,
            strum::VariantNames,
        )]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize),
            serde(rename_all = $lit)
        )]
        #[strum(serialize_all = $lit)]
        $vis enum $name $($rest)*
    };
}

error_kind! {
    pub enum SplineError {
        ShapeError,
        TooFewKnots,
        NotEnoughPoints,
    }
}

error_kind! {
    pub enum ShapeError {
        DegreeMismatch,
        NotEnoughtKnots {
            expected: usize,
            found: usize,
        },
        NotEnoughPoints,
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SplineError {}
