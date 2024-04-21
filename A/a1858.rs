use std::io::stdin;

fn main() {
    let mut input_line = String::new();
    stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let n: i32 = input_line.trim().parse().expect("invalid input");

    for _ in 0..n {
        solve();
    }
}

fn solve() {
    let mut input_line = String::new();
    stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let numbers: Vec<i32> = input_line
        .split_whitespace()
        .map(|num| num.parse().expect("Please enter a valid number"))
        .collect();
    let a = numbers[0];
    let b = numbers[1];
    let c = numbers[2] % 2;
    if a + c > b {
        println!("First")
    } else {
        println!("Second")
    }
}
