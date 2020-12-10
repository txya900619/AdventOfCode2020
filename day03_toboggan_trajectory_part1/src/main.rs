use std::io::{stdin, BufRead};

fn main() {
    let mut map: Vec<String> = Vec::new();
    let mut count = 0;
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        if s == "stop" {
            break;
        }
        map.push(s);
    }

    for (i, row) in map.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if row.chars().collect::<Vec<char>>()[i * 3 % row.len()] == '#' {
            count += 1;
        }
    }

    print!("{}\n", count);
}
