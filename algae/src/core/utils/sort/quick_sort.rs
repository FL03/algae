/*
   Appellation: quick_sort <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

/// Implement a quick sorting algorithm
#[derive(Clone, Debug, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct QuickSort<T>(T);

impl<T: std::cmp::PartialOrd + Ord> QuickSort<T> {
    fn _quick_sort(arr: &mut [T], lo: isize, hi: isize) -> &mut [T] {
        if lo < hi {
            let p = Self::partition(arr, lo, hi);
            Self::_quick_sort(arr, lo, p - 1);
            Self::_quick_sort(arr, p + 1, hi);
        }
        arr
    }
    fn partition(arr: &mut [T], lo: isize, hi: isize) -> isize {
        let pivot = hi as usize;
        let mut i = lo - 1;
        let mut j = hi;

        loop {
            i += 1;
            while arr[i as usize] < arr[pivot] {
                i += 1;
            }
            j -= 1;
            while j >= 0 && arr[j as usize] > arr[pivot] {
                j -= 1;
            }
            if i >= j {
                break;
            } else {
                arr.swap(i as usize, j as usize);
            }
        }

        arr.swap(i as usize, pivot as usize);
        i
    }
    pub fn new(arr: &mut [T]) -> &mut [T] {
        let len = arr.len();
        Self::_quick_sort(arr, 0, (len - 1) as isize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut to_sort = [1, 3, 2, 7, 4, 5];
        let expected: [i32; 6] = [1, 2, 3, 4, 5, 7];
        assert_eq!(&expected, QuickSort::new(&mut to_sort))
    }
}
