/*
   Appellation: edit_distance <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use serde::{Deserialize, Serialize};
use std::cmp::min;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EditDistance {
    pub data: (String, String),
}

impl EditDistance {
    pub fn new(data: (String, String)) -> Self {
        Self { data }
    }
    pub fn from(data: (&str, &str)) -> Self {
        Self::new((String::from(data.0), String::from(data.1)))
    }
    pub fn distance_se(self) -> u32 {
        let (str_a, str_b) = (self.data.0.as_bytes(), self.data.1.as_bytes());
        let (m, n) = (str_a.len(), str_b.len());
        let mut distances: Vec<u32> = vec![0; n + 1];
        let mut s: u32;
        let mut c: u32;
        let mut char_a: u8;
        let mut char_b: u8;

        for (j, v) in distances.iter_mut().enumerate().take(n + 1).skip(1) {
            *v = j as u32;
        }
        for i in 1..=m {
            s = (i - 1) as u32;
            c = i as u32;
            char_a = str_a[i - 1];
            for j in 1..=n {
                char_b = str_b[j - 1];
                c = min(
                    s + if char_a == char_b { 0 } else { 1 },
                    min(c + 1, distances[j] + 1),
                );
                s = distances[j];
                distances[j] = c;
            }
        }

        distances[n]
    }
    pub fn distance(self) -> u32 {
        let (str_a, str_b): (&str, &str) = (&*self.data.0, &*self.data.1);
        let mut distances = vec![vec![0; str_b.len() + 1]; str_a.len() + 1];
        for j in 0..=str_b.len() {
            distances[0][j] = j as u32;
        }
        for (i, item) in distances.iter_mut().enumerate() {
            item[0] = i as u32;
        }

        for i in 1..=str_a.len() {
            for j in 1..=str_b.len() {
                distances[i][j] = min(distances[i - 1][j] + 1, distances[i][j - 1] + 1);
                if str_a.as_bytes()[i - 1] == str_b.as_bytes()[j - 1] {
                    distances[i][j] = min(distances[i][j], distances[i - 1][j - 1]);
                } else {
                    distances[i][j] = min(distances[i][j], distances[i - 1][j - 1] + 1);
                }
            }
        }
        distances[str_a.len()][str_b.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_strings() {
        assert_eq!(
            0,
            EditDistance::from(("Hello, world!", "Hello, world!")).distance()
        );
        assert_eq!(
            0,
            EditDistance::from(("Hello, world!", "Hello, world!")).distance_se()
        );
        assert_eq!(
            0,
            EditDistance::from(("Test_Case_#1", "Test_Case_#1")).distance()
        );
        assert_eq!(
            0,
            EditDistance::from(("Test_Case_#1", "Test_Case_#1")).distance_se()
        );
    }

    #[test]
    fn one_edit_difference() {
        assert_eq!(
            1,
            EditDistance::from(("Hello, world", "Hello, world!")).distance()
        );
        assert_eq!(
            1,
            EditDistance::from(("Hello, world", "Hello, world!")).distance_se()
        );
        assert_eq!(
            1,
            EditDistance::from(("Test_Case_#", "Test_Case_#1")).distance()
        );
        assert_eq!(
            1,
            EditDistance::from(("Test_Case_#", "Test_Case_#1")).distance_se()
        );
    }

    #[test]
    fn several_differences() {
        assert_eq!(8, EditDistance::from(("Hello", "Hello, world!")).distance());
        assert_eq!(
            8,
            EditDistance::from(("Hello", "Hello, world!")).distance_se()
        );
        assert_eq!(
            3,
            EditDistance::from(("Test_Case", "Test_Case_#1")).distance()
        );
        assert_eq!(
            3,
            EditDistance::from(("Test_Case", "Test_Case_#1")).distance_se()
        );
    }
}
