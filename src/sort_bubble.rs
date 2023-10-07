pub fn sort_bubble<T: Ord + Clone>(arr: &[T], is_asc: bool) -> Vec<T> {
    let mut result = arr.to_vec();
    let n = result.len();

    for i in 0..n {
        for j in 0..n - 1 - i {
            if (is_asc && result[j] > result[j + 1]) || (!is_asc && result[j] < result[j + 1]) {
                result.swap(j, j + 1);
            }
        }
    }

    result
}

pub fn sort_bubble_inplace<T: Ord + Clone>(arr: &mut [T], is_asc: bool) {
    let n = arr.len();

    for i in 0..n {
        for j in 0..n - 1 - i {
            if (is_asc && arr[j] > arr[j + 1]) || (!is_asc && arr[j] < arr[j + 1]) {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascending_sort() {
        let vec = vec![5, 2, 9, 1, 5, 6];
        assert_eq!(sort_bubble(&vec, true), vec![1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn test_descending_sort() {
        let vec = vec![5, 2, 9, 1, 5, 6];
        assert_eq!(sort_bubble(&vec, false), vec![9, 6, 5, 5, 2, 1]);
    }

    #[test]
    fn test_empty_array() {
        let vec: Vec<i32> = vec![];
        assert_eq!(sort_bubble(&vec, false), vec![]);
        assert_eq!(sort_bubble(&vec, true), vec![]);
    }

    #[test]
    fn test_single_element() {
        let vec = vec![314159265];
        assert_eq!(sort_bubble(&vec, true), vec![314159265]);
        assert_eq!(sort_bubble(&vec, false), vec![314159265]);
    }

    #[test]
    fn test_already_sorted() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(sort_bubble(&vec, true), vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_inplace_ascending_sort() {
        let mut vec = vec![5, 2, 9, 1, 5, 6];
        sort_bubble_inplace(&mut vec, true);
        assert_eq!(vec, vec![1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn test_inplace_descending_sort() {
        let mut vec = vec![5, 2, 9, 1, 5, 6];
        sort_bubble_inplace(&mut vec, false);
        assert_eq!(vec, vec![9, 6, 5, 5, 2, 1]);
    }

    #[test]
    fn test_inplace_empty_array() {
        let mut vec: Vec<i32> = vec![];
        sort_bubble_inplace(&mut vec, false);
        assert_eq!(vec, vec![]);
        sort_bubble_inplace(&mut vec, true);
        assert_eq!(vec, vec![]);
    }

    #[test]
    fn test_inplace_single_element() {
        let mut vec = vec![314159265];
        sort_bubble_inplace(&mut vec, true);
        assert_eq!(vec, vec![314159265]);
        sort_bubble_inplace(&mut vec, false);
        assert_eq!(vec, vec![314159265]);
    }

    #[test]
    fn test_inplace_already_sorted() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
        sort_bubble_inplace(&mut vec, true);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
