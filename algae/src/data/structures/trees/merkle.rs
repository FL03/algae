/*
   Appellation: merkle
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn constructor(keys: Vec<T>, children: Vec<Node<T>>) -> Self {
        Self { keys, children }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct MerkleTree;

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
