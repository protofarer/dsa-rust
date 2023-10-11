pub fn sort_quick<T: Ord + Clone>(arr: &mut [T], lo: usize, hi: usize, is_asc: bool) {
    if hi > lo {
        let p: usize = partition(arr, lo, hi, is_asc);
        if p > 0 {
            sort_quick(arr, lo, p - 1, is_asc);
        }
        sort_quick(arr, p + 1, hi, is_asc);
    }
}

fn partition<T: Ord + Clone>(arr: &mut [T], lo: usize, hi: usize, is_asc: bool) -> usize {
    let p = hi;
    let mut first_high = lo;
    for i in lo..hi {
        if (is_asc && arr[i] < arr[p]) || (!is_asc && arr[i] > arr[p]) {
            arr.swap(i, first_high);
            first_high += 1;
        }
    }

    arr.swap(p, first_high);
    first_high
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending_sort() {
        let mut vec = vec![5, 2, 9, 1, 5, 6];
        let hi = vec.len() - 1;
        sort_quick(&mut vec, 0, hi, true);
        assert_eq!(vec, vec![1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn test_descending_sort() {
        let mut vec = vec![5, 2, 9, 1, 5, 6];
        let hi = vec.len() - 1;
        sort_quick(&mut vec, 0, hi, false);
        assert_eq!(vec, vec![9, 6, 5, 5, 2, 1]);
    }

    #[test]
    fn test_empty_array() {
        let mut vec: Vec<i32> = vec![];
        let test: Vec<i32> = vec![];
        let hi = if vec.len() == 0 { 0 } else { vec.len() - 1 };

        sort_quick(&mut vec, 0, hi, true);
        assert_eq!(vec, test);

        sort_quick(&mut vec, 0, hi, false);
        assert_eq!(vec, test);
    }

    #[test]
    fn test_single_element() {
        let mut vec = vec![314159265];
        let hi = vec.len() - 1;

        sort_quick(&mut vec, 0, hi, true);
        assert_eq!(vec, vec![314159265]);

        sort_quick(&mut vec, 0, hi, false);
        assert_eq!(vec, vec![314159265]);
    }

    #[test]
    fn test_already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
        let hi = vec.len() - 1;
        sort_quick(&mut vec, 0, hi, true);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
