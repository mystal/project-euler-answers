use std::env;
use std::ops::Range;
use std::str::FromStr;

fn main() {
    let arg = env::args().nth(1).unwrap();
    let n = FromStr::from_str(&arg).unwrap();
    println!("{:?}", prime_factors(n));
}

fn prime_factors(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let mut cur = n;
    loop {
        if let Some(factor) = prime_candidates(cur).find(|&f| cur % f == 0) {
            factors.push(factor);
            cur /= factor;
        } else {
            break;
        }
    }
    factors.push(cur);
    factors
}

fn prime_candidates(n: i64) -> Range<i64> {
    let top = n / 2;
    2..top + 1
}
