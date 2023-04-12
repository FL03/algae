/*
    Appellation: rangemap <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::prelude::{Hashable, H256};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hashable, PartialEq, Serialize)]
pub struct Payload<T = String>
where
    T: ToString,
{
    id: H256,
    data: HashMap<H256, T>,
}

impl<T> Payload<T>
where
    T: ToString,
{
    pub fn new() -> Self {
        Self {
            id: H256::generate(),
            data: HashMap::new(),
        }
    }
    pub fn data(&self) -> &HashMap<H256, T> {
        &self.data
    }
    pub fn set_id(mut self, id: H256) -> Self {
        self.id = id;
        self
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            id: H256::generate(),
            data: HashMap::with_capacity(capacity),
        }
    }
}

impl<T> From<HashMap<H256, T>> for Payload<T>
where
    T: ToString,
{
    fn from(data: HashMap<H256, T>) -> Self {
        Self {
            id: H256::generate(),
            data,
        }
    }
}

impl<T> std::fmt::Display for Payload<T>
where
    T: ToString,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self
            .data
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<HashMap<String, String>>();
        write!(f, "{}", serde_json::to_value(data).unwrap())
    }
}

impl<T> Extend<T> for Payload<T>
where
    T: ToString,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for v in iter {
            self.data.insert(H256::generate(), v);
        }
    }
}

impl<T> Extend<(H256, T)> for Payload<T>
where
    T: ToString,
{
    fn extend<I: IntoIterator<Item = (H256, T)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.data.insert(k, v);
        }
    }
}

impl<T> std::ops::Index<H256> for Payload<T>
where
    T: ToString,
{
    type Output = T;

    fn index(&self, index: H256) -> &Self::Output {
        &self.data[&index]
    }
}

impl<T> std::ops::IndexMut<H256> for Payload<T>
where
    T: ToString,
{
    fn index_mut(&mut self, index: H256) -> &mut Self::Output {
        self.data.get_mut(&index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload() {
        let mut payload = Payload::new();
        payload.extend(vec!["a", "b"]);
        assert_eq!(payload.data().len(), 2);
    }
}
