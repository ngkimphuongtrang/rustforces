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

    // parse input_line to only one number
    let n: i32 = input_line.trim().parse().expect("Please enter a valid number");
    let mut max_n = 0;
    let mut res = 0;
    for x in 2..n+1{
        let m = n/x;
        if m >= max_n{
            max_n = m;
            res = x;
        } else{
            break;
        }
    }
    println!("{}", res);
}

/*
Author tutorial
To maximize the number of multiples of 𝑥 less than 𝑛
, it optimal to choose a small 𝑥
, in this case, 2
. The only exception is 𝑛=3
, where it is optimal to choose 3
 instead, since both 2
 and 3
 have only one multiple less than 3
.
*/
