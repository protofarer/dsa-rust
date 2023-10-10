// in place
pub fn sort_insertion<T: Ord + Clone>(arr: &mut [T], is_asc: bool) {
    let n = arr.len();
    if n == 1 {
        return;
    }

    for i in 1..n {
        for j in (1..i + 1).rev() {
            if (is_asc && arr[j] < arr[j - 1]) || (!is_asc && arr[j] > arr[j - 1]) {
                arr.swap(j - 1, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ascending_sort() {
        let mut vec = vec![5, 2, 9, 1, 5, 6];
        sort_insertion(&mut vec, true);
        assert_eq!(vec, vec![1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn test_descending_sort() {
        let mut vec = vec![5, 2, 9, 1, 5, 6];
        sort_insertion(&mut vec, false);
        assert_eq!(vec, vec![9, 6, 5, 5, 2, 1]);
    }

    #[test]
    fn test_empty_array() {
        let mut vec: Vec<i32> = vec![];
        sort_insertion(&mut vec, false);
        assert_eq!(vec, vec![]);
        sort_insertion(&mut vec, true);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut vec = vec![314159265];
        sort_insertion(&mut vec, true);
        assert_eq!(vec, vec![314159265]);

        sort_insertion(&mut vec, false);
        assert_eq!(vec, vec![314159265]);
    }

    #[test]
    fn test_already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
        sort_insertion(&mut vec, true);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    // #[test]
    // fn test_inplace_ascending_sort() {
    //     let mut vec = vec![5, 2, 9, 1, 5, 6];
    //     sort_insertion_inplace(&mut vec, true);
    //     assert_eq!(vec, vec![1, 2, 5, 5, 6, 9]);
    // }

    // #[test]
    // fn test_inplace_descending_sort() {
    //     let mut vec = vec![5, 2, 9, 1, 5, 6];
    //     sort_insertion_inplace(&mut vec, false);
    //     assert_eq!(vec, vec![9, 6, 5, 5, 2, 1]);
    // }

    // #[test]
    // fn test_inplace_empty_array() {
    //     let mut vec: Vec<i32> = vec![];
    //     sort_insertion_inplace(&mut vec, false);
    //     assert_eq!(vec, vec![]);
    //     sort_insertion_inplace(&mut vec, true);
    //     assert_eq!(vec, vec![]);
    // }

    // #[test]
    // fn test_inplace_single_element() {
    //     let mut vec = vec![314159265];
    //     sort_insertion_inplace(&mut vec, true);
    //     assert_eq!(vec, vec![314159265]);
    //     sort_insertion_inplace(&mut vec, false);
    //     assert_eq!(vec, vec![314159265]);
    // }

    // #[test]
    // fn test_inplace_already_sorted() {
    //     let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
    //     sort_insertion_inplace(&mut vec, true);
    //     assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7]);
    // }
}
