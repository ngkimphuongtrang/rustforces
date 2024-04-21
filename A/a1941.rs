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
    let numbers: Vec<i32> = input_line
        .split_whitespace()
        .map(|num| num.parse().expect("Please enter a valid number"))
        .collect();

    let n = numbers[0];
    let m = numbers[1];
    let k = numbers[2];

    let mut input_line = String::new();
    stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
    let b: Vec<i32> = input_line.split_whitespace().map(|num| num.parse().expect("Please enter a valid number")).collect();
    let mut input_line = String::new();
    stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
    let c: Vec<i32> = input_line.split_whitespace().map(|num| num.parse().expect("Please enter a valid number")).collect();

    let mut result = 0;
    for i in 0..n{
        for j in 0..m{
            if b[i as usize] + c[j as usize] <= k{
                result+=1
            }
        }
    }
    println!("{}",result);
}
