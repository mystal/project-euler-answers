extern crate num;

fn main() {
    let n = 2;
    println!("{}", largest_palindrome_product(n))
}

fn largest_palindrome_product(n: u64) -> u64 {
    let mut bottom = num::pow(10, n - 1);
    let mut top = num::pow(10, n);
    let mut largest = 0;

    for i in bottom..top {
        for j in i..top {
            let product = i * j;
            if product > largest && is_palindrome(&product.to_str()) {
                largest = product;
            }
        }
    }
    largest
}

fn is_palindrome(s: &str) -> bool {
    let mut forward = s.chars().enumerate();
    let mut backward = s.chars().enumerate().rev();
    //let mut left = 0;
    //let mut right = s.char_len() - 1;

    while left < s.char_len() / 2 && right >= s.char_len() / 2 {
        if s[left] != s[right] {
            return false;
        }
    }
    true
}
