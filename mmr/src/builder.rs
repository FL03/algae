/*
    Appellation: builder <mmr>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::MerkleMountainRange;

pub struct MerkleMountainRangeBuilder<T>
where
    T: ToString,
{
    pub mmr: MerkleMountainRange<T>,
}
