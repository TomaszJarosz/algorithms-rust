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
#[cfg(test)]
mod tests {
    use crate::algos::array::is_palindromic;
    use crate::algos::array::is_palindromic_iter;
    use crate::algos::array::two_sum_sorted;

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
}