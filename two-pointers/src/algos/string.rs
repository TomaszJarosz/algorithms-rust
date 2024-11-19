/// Checks if a string is palindromic (reads the same forwards and backwards).
///
/// # Examples
/// ```
/// assert!(is_palindromic("madam"));
/// assert!(!is_palindromic("hello"));
/// ```
pub fn is_palindromic(string: &str) -> bool {
    let chars = string.chars().collect::<Vec<char>>();
    let mut left = 0;
    let mut right = chars.len().saturating_sub(1);
    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right = right.saturating_sub(1);
    }
    true
}

/// Checks if a string is palindromic using iterators.
/// This version is more concise but may have different performance characteristics.
///
/// # Examples
/// ```
/// assert!(is_palindromic_string_iters("madam"));
/// assert!(!is_palindromic_string_iters("hello"));
/// ```
pub fn is_palindromic_string_iters(string: &str) -> bool {
    string.chars().eq(string.chars().rev())
}

/// Checks if string `s` is a subsequence of string `t`.
/// A subsequence is derived by deleting some or no elements of `t` without changing the order of the remaining elements.
///
/// # Examples
/// ```
/// assert!(is_subsequence("ace", "abcde"));
/// assert!(!is_subsequence("aec", "abcde"));
/// ```
pub fn is_subsequence(s: &str, t: &str) -> bool {
    let mut s_iter = s.chars();
    let mut current_char = s_iter.next();

    for t_char in t.chars() {
        if current_char == Some(t_char) {
            current_char = s_iter.next();
        }
        if current_char.is_none() {
            return true;
        }
    }

    current_char.is_none()
}

/// Reverses a vector of characters in place.
///
/// # Examples
/// ```
/// let mut s = vec!['h', 'e', 'l', 'l', 'o'];
/// reverse_string(&mut s);
/// assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
/// ```
pub fn reverse_string(s: &mut Vec<char>) {
    let mut left = 0;
    let mut right = s.len().saturating_sub(1);
    while left < right {
        s.swap(left, right);
        left += 1;
        right = right.saturating_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindromic_string() {
        assert!(is_palindromic("aabbccbbaa"));
        assert!(is_palindromic("madam"));
        assert!(is_palindromic("racecar"));
        assert!(is_palindromic_string_iters("aabbccbbaa"));
        assert!(is_palindromic_string_iters("madam"));
        assert!(is_palindromic_string_iters("racecar"));
    }

    #[test]
    fn test_non_palindromic_string() {
        assert!(!is_palindromic("greeting"));
        assert!(!is_palindromic("aabccbbaa"));
        assert!(!is_palindromic("hello"));
        assert!(!is_palindromic_string_iters("greeting"));
        assert!(!is_palindromic_string_iters("aabccbbaa"));
        assert!(!is_palindromic_string_iters("hello"));
    }

    #[test]
    fn test_is_subsequence() {
        assert!(is_subsequence("ace", "abcde"));
        assert!(is_subsequence("", "abcde"));
        assert!(is_subsequence("abc", "abc"));
        assert!(is_subsequence("aaa", "aabbaaa"));
        assert!(is_subsequence("a", "a"));
        assert!(is_subsequence("abc", "aabbcc"));
    }

    #[test]
    fn test_is_not_subsequence() {
        assert!(!is_subsequence("aec", "abcde"));
        assert!(!is_subsequence("abc", ""));
        assert!(!is_subsequence("abcdef", "abc"));
        assert!(!is_subsequence("a", "b"));
    }

    #[test]
    fn test_reverse_string() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);

        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);

        let mut s: Vec<char> = vec![];
        reverse_string(&mut s);
        assert_eq!(s, vec![]);
    }
}
