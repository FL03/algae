/*
    Appellation: btree <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{cmps::*, primitives::*, specs::*, utils::*};

pub(crate) mod cmps;
pub(crate) mod specs;
pub(crate) mod tree;

pub(crate) mod primitives {
    use super::BaseObj;
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;

    #[derive(
        Copy, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
    )]
    pub struct Data<T>(pub T);

    impl<T> Data<T> {
        pub fn new(data: T) -> Self {
            Self(data)
        }
        pub fn data(&self) -> &T {
            &self.0
        }
    }

    impl<T> From<&T> for Data<T>
    where
        T: Clone,
    {
        fn from(data: &T) -> Self {
            Self::new(data.clone())
        }
    }

    impl<T> BaseObj for Data<T> where T: Copy + Debug + Default + Ord {}
}

pub(crate) mod utils {}
