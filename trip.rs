use std::io;

fn trip(_a: Vec<i32>, _b: Vec<i32>) -> Vec<i32> {
    let (mut alice, mut bob, mut result): (i32, i32, Vec<i32>) = (0, 0, vec![]);

    for (pos, _) in _a.iter().enumerate() {
        if _a[pos] > _b[pos] {
            alice = alice + 1;
        } else if _a[pos] < _b[pos] {
            bob += 1;
        }
    }

    result.push(alice);
    result.push(bob);
    return result;
}

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a).expect("Err");
    io::stdin().read_line(&mut b).expect("Err");

    let a: Vec<i32> = a.split_whitespace()
    .map(|s| s.parse().expect("Err"))
    .collect();
    let b: Vec<i32> = b.split_whitespace()
    .map(|s| s.parse().expect("Err"))
    .collect();

    let result = trip(a, b);
    println!("{:?}", result);
}