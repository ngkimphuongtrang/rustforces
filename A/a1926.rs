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
    let mut s = String::new();
    let mut a = 0;
    let mut b = 0;

    stdin().read_line(&mut s).expect("Failed to read line");

    for c in s.chars() {
        match c {
            'A' => a += 1,
            'B' => b += 1,
            _ => {}
        }
    }

    if a > b {
        println!("A");
    } else {
        println!("B");
    }
}
