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
    let _n: i32 = input_line.trim().parse().expect("invalid input");

    let mut input_line = String::new();
    stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    // Split the input by whitespace and collect into a vector
    let numbers: Vec<i32> = input_line
        .split_whitespace()
        .map(|num| num.parse().expect("Please enter a valid number"))
        .collect();
    let mut sum = 0;
    for num in numbers {
        sum += num.abs();
    }
    println!("{}", sum)
}
