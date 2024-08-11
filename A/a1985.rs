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
    // Split the input by whitespace and collect into a vector
    let ins: Vec<String> = input_line
        .split_whitespace()
        .map(|num| num.parse().expect("Please enter a valid number"))
        .collect();

    let a = &ins[0].chars().collect::<Vec<char>>();
    let b = &ins[1].chars().collect::<Vec<char>>();

    print!("{}{}{} ", b[0], a[1], a[2]);
    println!("{}{}{}", a[0], b[1], b[2]);
}
