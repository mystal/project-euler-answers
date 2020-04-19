use std::os::args;

fn main() {
    let n = from_str::<uint>(args[1]).unwrap();
    let mut sum_of_squares = 0;
    let mut sum = 0;
    for i in range(1, n + 1) {
        sum_of_squares += i*i;
        sum += i;
    }
    println!("{}", sum*sum - sum_of_squares);
}

//use std::iter::AdditiveIterator;
//use std::os::args;
//
//fn main() {
//    //let n = from_str::<int>(args[1]).unwrap();
//    let n = 100u;
//    let mut r = range(1, n + 1);
//    let sum_of_squares = r.map(|i| i * i).sum();
//    let sum = r.sum();
//    println!("{}", sum*sum - sum_of_squares);
//}
