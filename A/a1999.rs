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

    let a = input_line.chars().collect::<Vec<char>>();

    let mut sum = 0;
    for d in a{
        let digit = d.to_digit(10).unwrap_or(0);
        sum += digit;
    }

    println!("{}", sum);
}
