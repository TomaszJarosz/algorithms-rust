mod algos;

fn main() {
    println!("aab is palindromic?: {}", algos::string::is_palindromic("aab"));
    println!("madam is palindromic?: {}", algos::string::is_palindromic("madam"));
    println!("racecar is palindromic?: {}", algos::string::is_palindromic("racecar"));
    println!("Ada is palindromic?: {}", algos::string::is_palindromic("ada"));

    println!("[1,2,3,4,5] is palindromic?: {}", algos::array::is_palindromic(&[1, 2, 3, 4, 5]));
    println!("[1,2,3,2,1] is palindromic?: {}", algos::array::is_palindromic(&[1, 2, 3, 2, 1]));
}


