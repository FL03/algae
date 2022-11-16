/*
    Appellation: store <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MMRStore<T>(pub T);

impl<T> MMRStore<T> {
    pub fn new(data: T) -> Self {
        Self(data)
    }
    pub fn data(&self) -> &T {
        &self.0
    }
}

impl<T> Hashable for MMRStore<T>
where
    T: Serialize,
{
    fn hash(&self) -> H256 {
        hasher(&self).as_slice().to_owned().into()
    }
}

impl<T> std::convert::From<&MMRStore<T>> for MMRStore<T>
where
    T: Clone,
{
    fn from(data: &MMRStore<T>) -> Self {
        Self(data.clone().0)
    }
}

impl<T> std::convert::From<T> for MMRStore<T> {
    fn from(data: T) -> Self {
        Self(data)
    }
}

impl<T> std::fmt::Display for MMRStore<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scsys::actors::generate::generate_random_string;
    use std::ops::Range;
    #[test]
    fn test_store_default() {
        let a = MMRStore::<usize>::default();
        let b = MMRStore::<usize>::new(Default::default());
        assert_eq!(a, b)
    }

    #[test]
    fn test_store_hash() {
        let data = Range { start: 0, end: 12 }
            .map(|_| generate_random_string(12))
            .collect::<Vec<String>>();

        let a = MMRStore::<Vec<String>>::from(data);
        let b = MMRStore::from(&a);
        assert_eq!(a, b)
    }
}
