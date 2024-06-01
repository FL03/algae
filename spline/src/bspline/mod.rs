/*
    Appellation: bspline <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::spline::*;

pub(crate) mod spline;

pub(crate) mod prelude {
    pub use super::spline::*;
}
