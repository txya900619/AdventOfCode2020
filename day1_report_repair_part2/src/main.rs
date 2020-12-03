use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut nums: Vec<i32> = Vec::new();
    for s in stdin.lock().lines() {
        let s = s.unwrap();
        if s == "stop" {
            break;
        }
        match s.parse::<i32>() {
            Ok(v) => nums.push(v),
            Err(e) => {
                print!("{}", e);
                break;
            }
        }
    }

    for num in &nums {
        for num2 in &nums {
            if num == num2 {
                continue;
            }
            for num3 in &nums {
                if num3 == num || num3 == num2 {
                    continue;
                }
                if num + num2 + num3 == 2020 {
                    print!("{}\n", num * num2 * num3);
                    return;
                }
            }
        }
    }
}
