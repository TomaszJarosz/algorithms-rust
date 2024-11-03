pub fn is_palindromic(array: &[i32]) -> bool {
    let mut left = 0;
    let mut right = array.len().saturating_sub(1);

    while left < right {
        if array[left] != array[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

pub fn is_palindromic_iter(array: &[i32]) -> bool {
    array.iter().eq(array.iter().rev())
}
/// Given a sorted array of unique integers and a target integer,
/// return true if there exists a pair of numbers that sum to target, false otherwise.
/// This problem is similar to Two Sum. (In Two Sum, the input is not sorted).
/// For example, given nums = [1, 2, 4, 6, 8, 9, 14, 15] and target = 13, return true because 4 + 9 = 13.
pub fn two_sum_sorted(sorted_array: &[i32], sum: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = sorted_array.len().saturating_sub(1);

    while left < right {
        let curr_sum = sorted_array[left] + sorted_array[right];
        if curr_sum == sum {
            return Some((left, right));
        } else if curr_sum > sum {
            right -= 1;
        } else {
            left += 1;
        }
    }
    None
}

/// Given two sorted integer arrays arr1 and arr2,
/// return a new array that combines both of them and is also sorted.
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::algos::array::{
        is_palindromic,
        is_palindromic_iter,
        merge_sorted_arrays,
        two_sum_sorted
    };

    #[test]
    fn test_palindromic_array() {
        assert!(is_palindromic(&[1, 2, 3, 2, 1]));
        assert!(is_palindromic(&[1, 2, 3, 3, 2, 1]));

        assert!(is_palindromic_iter(&[1, 2, 3, 2, 1]));
        assert!(is_palindromic_iter(&[1, 2, 3, 3, 2, 1]));
    }

    #[test]
    fn test_non_palindromic_array() {
        assert!(!is_palindromic(&[1, 2, 3, 4, 5]));
        assert!(!is_palindromic(&[1, 1, 3, 1]));

        assert!(!is_palindromic_iter(&[1, 2, 3, 4, 5]));
        assert!(!is_palindromic_iter(&[1, 1, 3, 1]));
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum_sorted(&[1, 2, 3, 4], 3), Some((0, 1)));
        assert_eq!(two_sum_sorted(&[1, 1, 3, 4], 2), Some((0, 1)));
        assert_eq!(two_sum_sorted(&[1, 3, 5, 6], 8), Some((1, 2)));
    }
    #[test]
    fn test_merge_two_non_empty_sorted_arrays() {
        let arr1 = [1, 3, 5, 7];
        let arr2 = [2, 4, 6, 8];
        let result = merge_sorted_arrays(&arr1, &arr2);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_merge_with_empty_array() {
        let arr1 = [1, 2, 3];
        let arr2: [i32; 0] = [];
        let result = merge_sorted_arrays(&arr1, &arr2);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_empty_arrays() {
        let arr1: [i32; 0] = [];
        let arr2: [i32; 0] = [];
        let result = merge_sorted_arrays(&arr1, &arr2);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_merge_with_duplicates() {
        let arr1 = [1, 2, 2, 3];
        let arr2 = [2, 3, 4];
        let result = merge_sorted_arrays(&arr1, &arr2);
        assert_eq!(result, vec![1, 2, 2, 2, 3, 3, 4]);
    }

    #[test]
    fn test_merge_with_negative_numbers() {
        let arr1 = [-5, -3, -1, 0];
        let arr2 = [-4, -2, 1, 2];
        let result = merge_sorted_arrays(&arr1, &arr2);
        assert_eq!(result, vec![-5, -4, -3, -2, -1, 0, 1, 2]);
    }

    #[test]
    fn test_merge_arrays_with_one_element() {
        let arr1 = [1];
        let arr2 = [2];
        let result = merge_sorted_arrays(&arr1, &arr2);
        assert_eq!(result, vec![1, 2]);
    }
}