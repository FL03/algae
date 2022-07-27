/*
   Appellation: edit_distance <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use std::cmp::min;

pub fn edit_distance(str_a: &str, str_b: &str) -> u32 {
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

pub fn edit_distance_se(str_a: &str, str_b: &str) -> u32 {
    let (str_a, str_b) = (str_a.as_bytes(), str_b.as_bytes());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_strings() {
        assert_eq!(0, edit_distance("Hello, world!", "Hello, world!"));
        assert_eq!(0, edit_distance_se("Hello, world!", "Hello, world!"));
        assert_eq!(0, edit_distance("Test_Case_#1", "Test_Case_#1"));
        assert_eq!(0, edit_distance_se("Test_Case_#1", "Test_Case_#1"));
    }

    #[test]
    fn one_edit_difference() {
        assert_eq!(1, edit_distance("Hello, world!", "Hell, world!"));
        assert_eq!(1, edit_distance("Test_Case_#1", "Test_Case_#2"));
        assert_eq!(1, edit_distance("Test_Case_#1", "Test_Case_#10"));
        assert_eq!(1, edit_distance_se("Hello, world!", "Hell, world!"));
        assert_eq!(1, edit_distance_se("Test_Case_#1", "Test_Case_#2"));
        assert_eq!(1, edit_distance_se("Test_Case_#1", "Test_Case_#10"));
    }

    #[test]
    fn several_differences() {
        assert_eq!(2, edit_distance("My Cat", "My Case"));
        assert_eq!(7, edit_distance("Hello, world!", "Goodbye, world!"));
        assert_eq!(6, edit_distance("Test_Case_#3", "Case #3"));
        assert_eq!(2, edit_distance_se("My Cat", "My Case"));
        assert_eq!(7, edit_distance_se("Hello, world!", "Goodbye, world!"));
        assert_eq!(6, edit_distance_se("Test_Case_#3", "Case #3"));
    }
}
