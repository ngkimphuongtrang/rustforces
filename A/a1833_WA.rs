use std::collections::HashMap;
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

    let mut map: HashMap<String, bool> = HashMap::new();
    let chars: Vec<(usize, char)> = input_line.char_indices().collect();
    for i in 0..chars.len() - 1 {
        let start = chars[i].0;
        let end = if i + 2 < chars.len() {
            chars[i + 2].0
        } else {
            chars.len()
        };
        if let Some(t) = input_line.get(start..end) {
            map.insert(t.to_string(), true);
        }
    }
    // -1 due to input_line.len() = n + 1
    println!("{}", map.len() - 1);
}
