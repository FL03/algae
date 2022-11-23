/*
    Appellation: builder <mmr>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::MerkleMountainRange;
use scsys::prelude::Hashable;

pub struct MerkleMountainRangeBuilder<T: Hashable> {
    pub mmr: MerkleMountainRange<T>,
}
