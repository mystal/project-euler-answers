use std::collections::HashMap;

const TOP: u64 = 1_000_000;

// TODO: Make this an iterator?
fn next(i: u64) -> u64 {
    if i % 2 == 0 {
        i / 2
    } else {
        (3 * i) + 1
    }
}

// TODO: Actually implement caching?
pub fn collatz_sequence_len(mut n: u64/*, cache: HashMap<u64, u64>*/) -> u64 {
    let mut len = 1;
    while n != 1 {
        len += 1;
        n = next(n);
    }
    len
    //while !cache.contains_key(n) {
    //    len += 1;
    //    n = next(n);
    //}
    //cache[n] + len
}

fn main() {
    //let mut cache = HashMap::new();
    //cache.insert(1, 1);

    let mut max = (0, 0);
    for i in (1..TOP).rev() {
        let len = collatz_sequence_len(i);
        //println!("{}: {}", i, len);
        if len > max.0 {
            max = (len, i);
        }
    }

    println!("n: {}\nlen: {}", max.1, max.0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn collatz_sequence_len_13() {
        assert_eq!(collatz_sequence_len(13), 10);
    }
}
