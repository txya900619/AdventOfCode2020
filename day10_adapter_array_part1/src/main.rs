use std::io::{stdin, BufRead};

fn main() {
    let mut nums: Vec<u32> = Vec::new();
    let mut current = 0;
    let mut one_jolts = 0;
    let mut three_jolts = 0;

    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s_trim = s.trim();
        if s_trim == "stop" {
            break;
        }
        nums.push(s_trim.parse().unwrap())
    }
    nums.sort();

    for num in nums {
        if num - current == 1 {
            one_jolts += 1;
        }
        if num - current == 3 {
            three_jolts += 1;
        }
        current = num;
    }

    print!("{}\n", one_jolts * (three_jolts + 1));
}
