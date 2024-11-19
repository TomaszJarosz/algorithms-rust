/// Checks if an array is palindromic.
/// A palindromic array reads the same forward and backward.
///
/// # Examples
/// ```
/// assert!(is_palindromic(&[1, 2, 3, 2, 1]));
/// assert!(!is_palindromic(&[1, 2, 3, 4, 5]));
/// ```
pub fn is_palindromic(array: &[i32]) -> bool {
    let mut left = 0;
    let mut right = array.len().saturating_sub(1);

    while left < right {
        if array[left] != array[right] {
            return false;
        }
        left += 1;
        right = right.saturating_sub(1);
    }
    true
}

/// Checks if an array is palindromic using iterators.
/// This version is more concise but may have different performance characteristics.
///
/// # Examples
/// ```
/// assert!(is_palindromic_iter(&[1, 2, 3, 2, 1]));
/// assert!(!is_palindromic_iter(&[1, 2, 3, 4, 5]));
/// ```
pub fn is_palindromic_iter(array: &[i32]) -> bool {
    array.iter().eq(array.iter().rev())
}

/// Given a sorted array of unique integers and a target integer,
/// return the indices of a pair of numbers that sum to the target, or `None` if no such pair exists.
///
/// # Examples
/// ```
/// assert_eq!(two_sum_sorted(&[1, 2, 4, 6, 8, 9, 14, 15], 13), Some((2, 5)));
/// assert_eq!(two_sum_sorted(&[1, 3, 5], 10), None);
/// ```
pub fn two_sum_sorted(sorted_array: &[i32], target_sum: i32) -> Option<(usize, usize)> {
    let mut left = 0;
    let mut right = sorted_array.len().saturating_sub(1);

    while left < right {
        let curr_sum = sorted_array[left] + sorted_array[right];
        if curr_sum == target_sum {
            return Some((left, right));
        } else if curr_sum > target_sum {
            right = right.saturating_sub(1);
        } else {
            left += 1;
        }
    }
    None
}

/// Merges two sorted integer arrays into a single sorted array.
///
/// # Examples
/// ```
/// let arr1 = [1, 3, 5];
/// let arr2 = [2, 4, 6];
/// assert_eq!(merge_sorted_arrays(&arr1, &arr2), vec![1, 2, 3, 4, 5, 6]);
/// ```
pub fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
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

/// Given an integer array sorted in non-decreasing order,
/// returns an array of the squares of each number, sorted in non-decreasing order.
///
/// # Examples
/// ```
/// let nums = vec![-4, -1, 0, 3, 10];
/// assert_eq!(sorted_squares(nums), vec![0, 1, 9, 16, 100]);
/// ```
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }

    let mut result = vec![0; nums.len()];
    let (mut left, mut right) = (0, nums.len().saturating_sub(1));
    let mut write_pos = nums.len().saturating_sub(1);

    while left <= right {
        let left_square = nums[left] * nums[left];
        let right_square = nums[right] * nums[right];

        if left_square > right_square {
            result[write_pos] = left_square;
            left += 1;
        } else {
            result[write_pos] = right_square;
            right = right.saturating_sub(1);
        }

        if write_pos > 0 {
            write_pos = write_pos.saturating_sub(1);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(two_sum_sorted(&[1, 2, 4, 6, 8, 9, 14, 15], 13), Some((2, 5)));
        assert_eq!(two_sum_sorted(&[1, 3, 5], 10), None);
    }

    #[test]
    fn test_merge_two_sorted_arrays() {
        let arr1 = [1, 3, 5, 7];
        let arr2 = [2, 4, 6, 8];
        assert_eq!(merge_sorted_arrays(&arr1, &arr2), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_merge_with_empty_array() {
        let arr1 = [1, 2, 3];
        let arr2: [i32; 0] = [];
        assert_eq!(merge_sorted_arrays(&arr1, &arr2), vec![1, 2, 3]);
    }

    #[test]
    fn test_sorted_squares_example1() {
        let nums = vec![-4, -1, 0, 3, 10];
        assert_eq!(sorted_squares(nums), vec![0, 1, 9, 16, 100]);
    }

    #[test]
    fn test_sorted_squares_example2() {
        let nums = vec![-7, -3, 2, 3, 11];
        assert_eq!(sorted_squares(nums), vec![4, 9, 9, 49, 121]);
    }
}
