use std::io::{stdin, BufRead};

fn main() {
    let mut numbers: Vec<u64> = Vec::new();
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s_trim = s.trim();
        if s_trim == "stop" {
            break;
        }
        numbers.push(s_trim.parse::<u64>().unwrap())
    }
    'outer: for (i, num) in numbers.iter().enumerate() {
        if i < 25 {
            continue;
        }
        for (first_index, first) in numbers[i - 25..i].iter().enumerate() {
            for second in numbers[first_index + 1..i].iter() {
                if *num == first + second {
                    continue 'outer;
                }
            }
        }
        print!("{}\n", num);
        break;
    }
}
