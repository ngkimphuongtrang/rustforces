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
    stdin().read_line(&mut s).expect("Failed to read line");
    let hour = &s[0..2].parse::<i32>().unwrap();
    if *hour == 12 {
        println!("12:{} PM", &s[3..5]);
        return;
    }
    if *hour == 0 {
        println!("12:{} AM", &s[3..5]);
        return;
    }
    if *hour > 12 {
        if *hour - 12 < 10 {
            println!("0{}:{} PM", hour - 12, &s[3..5]);
        } else {
            println!("{}:{} PM", hour - 12, &s[3..5]);
        }
        return;
    }
    if *hour < 10 {
        println!("0{}:{} AM", hour, &s[3..5]);
    } else {
        println!("{}:{} AM", hour, &s[3..5]);
    }
}
