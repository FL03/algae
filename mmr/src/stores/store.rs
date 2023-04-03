/*
    Appellation: store <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::prelude::{Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Hashable, PartialEq, Serialize)]
pub struct MMRStore<T = String>
where
    T: ToString,
{
    id: H256,
    data: T,
}

impl<T> MMRStore<T>
where
    T: ToString,
{
    pub fn new(data: T) -> Self {
        Self {
            id: H256::generate(),
            data,
        }
    }
    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<T> From<T> for MMRStore<T>
where
    T: ToString,
{
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<T> std::fmt::Display for MMRStore<T>
where
    T: ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = serde_json::json!({
            "id": self.id,
            "data": self.data.to_string(),
        });
        write!(f, "{}", msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_default() {
        let a = MMRStore::<usize>::default();
        let b = MMRStore::<usize>::new(Default::default());
        assert_eq!(a, b)
    }
}
