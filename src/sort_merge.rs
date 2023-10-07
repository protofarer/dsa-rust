// todo in-place
// pub fn sort_merge_in_place<T: Ord + Clone>(
//     arr: &mut Vec<T>,
// ) {}

// out-of-place, default
pub fn sort_merge<T: Ord + Clone>(arr: &[T], is_asc: bool) -> Vec<T> {
    divide(arr, is_asc)
}

fn divide<T: Ord + Clone>(arr: &[T], is_asc: bool) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_owned();
    }

    let m = arr.len() / 2;
    let (left, right) = arr.split_at(m);
    let left_sorted = divide(left, is_asc);
    let right_sorted = divide(right, is_asc);
    merge(&left_sorted, &right_sorted, is_asc)
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], is_asc: bool) -> Vec<T> {
    // ! Not idiomatic, use iterative approach instead
    // let mut merged = Vec::new();
    // let mut i = 0;
    // let mut j = 0;

    // if is_asc {
    //     while i < left.len() && j < right.len() {
    //         if left[i] < right[j] {
    //             merged.push(left[i]);
    //             i += 1;
    //         } else {
    //             merged.push(right[j]);
    //             j += 1;
    //         }
    //     }
    // } else {
    //     while i < left.len() && j < right.len() {
    //         if left[i] >= right[j] {
    //             merged.push(left[i]);
    //             i += 1;
    //         } else {
    //             merged.push(right[j]);
    //             j += 1;
    //         }
    //     }
    // }
    // while i < left.len() {
    //     merged.push(left[i]);
    //     i += 1;
    // }
    // while j < right.len() {
    //     merged.push(right[j]);
    //     j += 1;
    // }
    // merged

    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();

    while let (Some(&left_val), Some(&right_val)) = (left_iter.peek(), right_iter.peek()) {
        if is_asc && left_val <= right_val || !is_asc && left_val >= right_val {
            merged.push((left_iter.next().unwrap()).clone()); // deref coercion, no need to explicit `*left_iter...`
        } else {
            merged.push((right_iter.next().unwrap()).clone());
        }
    }

    merged.extend(left_iter.cloned());
    merged.extend(right_iter.cloned());

    merged
}

#[cfg(test)]
mod tests {
    use super::sort_merge;
    use quickcheck::quickcheck;
    // Randomized, property based test
    quickcheck! {
        fn prop_sort_merge_like_std_sort(xs: Vec<i32>) -> bool {
            let mut std_sorted = xs.clone();
            std_sorted.sort();
            sort_merge(&xs, true) == std_sorted
        }
    }

    #[test]
    fn test_empty_vec() {
        let vec: Vec<i32> = Vec::new();
        assert_eq!(sort_merge(&vec, true), Vec::<i32>::new());
    }

    #[test]
    fn test_single_element() {
        let vec = vec![5];
        assert_eq!(sort_merge(&vec, true), vec![5]);
    }

    #[test]
    fn test_multiple_elements_ascending() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let sorted = vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9];
        assert_eq!(sort_merge(&vec, true), sorted);
    }

    #[test]
    fn test_multiple_elements_descending() {
        let vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let sorted = vec![9, 6, 5, 5, 5, 4, 3, 3, 2, 1, 1];
        assert_eq!(sort_merge(&vec, false), sorted);
    }

    #[test]
    fn test_sorted_input() {
        let vec = vec![1, 2, 3, 4, 5];
        assert_eq!(sort_merge(&vec, true), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted_input() {
        let vec = vec![5, 4, 3, 2, 1];
        assert_eq!(sort_merge(&vec, true), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_strings_ascending() {
        let vec = vec!["apple", "banana", "cherry", "date"];
        let sorted = vec!["apple", "banana", "cherry", "date"];
        assert_eq!(sort_merge(&vec, true), sorted);
    }

    #[test]
    fn test_strings_descending() {
        let vec = vec!["apple", "banana", "cherry", "date"];
        let sorted = vec!["date", "cherry", "banana", "apple"];
        assert_eq!(sort_merge(&vec, false), sorted);
    }
}
