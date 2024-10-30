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
#[cfg(test)]
mod tests {
    use crate::algos::array::is_palindromic;
    use crate::algos::array::is_palindromic_iter;

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
}