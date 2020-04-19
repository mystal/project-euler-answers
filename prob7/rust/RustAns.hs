use std::iter;

fn main() {
    let n = 10001;
    let mut primes = Vec::with_capacity(n);
    primes.push(2u);
    for i in iter::count(3u, 1u) {
        if !primes.iter().any(|p| i % *p == 0) {
            primes.push(i);
            if primes.len() == n {
                break;
            }
        }
    }
    println!("{}", primes.last().unwrap())
}
