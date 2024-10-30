pub fn is_palindromic(string: &str) -> bool {
    let chars: Vec<char> = string.chars().collect();
    let mut left = 0;
    let mut right = chars.len().saturating_sub(1);
    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
pub fn is_palindromic_string_iters(string: &str) -> bool {
    let chars: Vec<char> = string.chars().collect();
    chars.iter().eq(chars.iter().rev())
}
#[cfg(test)]
mod tests {
    use crate::algos::string::{is_palindromic, is_palindromic_string_iters};

    #[test]
    fn test_palindromic_string() {
        assert!(is_palindromic("aabbCCbbaa"));
        assert!(is_palindromic("madam"));
        assert!(is_palindromic("racecar"));

        assert!(is_palindromic_string_iters("aabbCCbbaa"));
        assert!(is_palindromic_string_iters("madam"));
        assert!(is_palindromic_string_iters("racecar"));
    }

    #[test]
    fn test_non_palindromic_string() {
        assert!(!is_palindromic("greeting"));
        assert!(!is_palindromic("aabCCbbaa"));
        assert!(!is_palindromic("aAaAaAaAaA"));

        assert!(!is_palindromic_string_iters("greeting"));
        assert!(!is_palindromic_string_iters("aabCCbbaa"));
        assert!(!is_palindromic_string_iters("aAaAaAaAaA"));
    }
}
