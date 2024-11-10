mod algos;

fn main() {
    println!("aab is palindromic?: {}", algos::string::is_palindromic("aab"));
    println!("madam is palindromic?: {}", algos::string::is_palindromic("madam"));
    println!("racecar is palindromic?: {}", algos::string::is_palindromic("racecar"));
    println!("Ada is palindromic?: {}", algos::string::is_palindromic("ada"));
    println!("Is `ace` subsequence of `abcde`,  Result {}", algos::string::is_subsequence("ace", "abcde"));


    println!("[1,2,3,4,5] is palindromic?: {}", algos::array::is_palindromic(&[1, 2, 3, 4, 5]));
    println!("[1,2,3,2,1] is palindromic?: {}", algos::array::is_palindromic(&[1, 2, 3, 2, 1]));
    println!("[1,3,5,6] ,  8 two_sum: {:?}", algos::array::two_sum_sorted(&[1, 3, 5, 6], 8).unwrap());
}
