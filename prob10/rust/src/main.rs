extern crate bit_vec;

use std::env;
use std::str::FromStr;

// TODO: finish me!
fn main() {
    let arg = env::args().nth(1).unwrap();
    let limit = FromStr::from_str(&arg).unwrap();

    let mut primes = bit_vec::BitVec::from_elem(limit, true);

    primes.set(0, false);
    primes.set(1, false);

    let bottom = 2;
    let top = 1 + (limit as f64).sqrt() as usize;
    let mut sum = 0;
    for i in bottom..top {
        if primes[i] {
            sum += i;
            for j in i.. {
                if i * j >= limit {
                    break;
                }
                primes.set(i * j, false);
            }
        }
    }
    for i in top..limit {
        if primes[i] {
            sum += i;
        }
    }
    println!("{}", sum)
}
