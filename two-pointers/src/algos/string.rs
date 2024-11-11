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

pub fn is_subsequence(s: &str, t: &str) -> bool {
    let (mut i, mut j) = (0, 0);
    let chars_s: Vec<char> = s.chars().collect();
    let chars_t: Vec<char> = t.chars().collect();

    while j < chars_t.len() {
        if i < chars_s.len() && chars_s[i] == chars_t[j] {
            i += 1;
        }
        j += 1;
    }

    i == chars_s.len()
}

pub fn reverse_string(s: &mut Vec<char>) {
    let mut left = 0;
    let mut right = s.len().saturating_sub(1);
    while left < right {
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::algos::string::{is_palindromic, is_palindromic_string_iters, is_subsequence, reverse_string};

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

    #[test]
    fn test_is_subsequence() {
        assert_eq!(is_subsequence("ace", "abcde"), true);
        assert_eq!(is_subsequence("", "abcde"), true);
        assert_eq!(is_subsequence("abc", "abc"), true);
        assert_eq!(is_subsequence("aaa", "aabbaaa"), true);
        assert_eq!(is_subsequence("a", "a"), true);
        assert_eq!(is_subsequence("abc", "aabbcc"), true);
    }

    #[test]
    fn test_is_not_subsequence() {
        assert_eq!(is_subsequence("aec", "abcde"), false);
        assert_eq!(is_subsequence("abc", ""), false);
        assert_eq!(is_subsequence("abcdef", "abc"), false);
        assert_eq!(is_subsequence("a", "b"), false);
    }

    #[test]
    fn test_reverse_string_example1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn test_reverse_string_example2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}
