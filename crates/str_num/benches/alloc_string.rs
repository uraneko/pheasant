use num_to_str::{num_arr, to_str};

pub fn main() {
    let num = 34526;
    let start = std::time::Instant::now();
    let arr = num_arr(num);
    let s = to_str(&arr);
    let dur = start.elapsed();

    println!("{:?}", s);
    println!("{:?}", dur);

    let start = std::time::Instant::now();
    let s = num.to_string();
    let dur = start.elapsed();
    println!("{:?}", s);
    println!("{:?}", dur);
}
