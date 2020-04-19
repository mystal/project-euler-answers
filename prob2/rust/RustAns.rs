fn main() {
    let mut first = 0;
    let mut second = 1;
    let limit = 4_000_000;
    let mut sum = 0;
    while second <= limit {
        if second % 2 == 0 {
            sum += second;
        }
        let third = first + second;
        first = second;
        second = third;
    }

    println!("{}", sum);
}
