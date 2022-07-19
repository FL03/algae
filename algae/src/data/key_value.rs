/*
    Appellation: key_value <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/



#[derive(Clone, Debug, Hash, PartialEq)]
pub struct KeyValue<T> {
    pub key: T,
    pub value: Vec<KeyValue<T>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
