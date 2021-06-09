use std::io;

fn main(){
    let mut numbers = String::new();

    io::stdin()
        .read_line(&mut numbers)
        .ok()
        .expect("read error");

    let numbers: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let _total: i32 = numbers.iter().sum();
    println!("{}", _total);
}
