use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut ans = 0;
    for s in stdin.lock().lines() {
        let s = s.unwrap();
        if s == "stop" {
            break;
        }
        if let [policy, password] = s.split(":").collect::<Vec<&str>>()[..] {
            if let [limits, keyword] = policy.split(" ").collect::<Vec<&str>>()[..] {
                let keyword = keyword.parse::<char>().unwrap();
                if let [min, max] = limits
                    .split("-")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()[..]
                {
                    let mut count = 0;
                    for c in password.trim().chars() {
                        if c == keyword {
                            count += 1;
                        }
                    }
                    if count >= min && count <= max {
                        ans += 1;
                    }
                }
            }
        }
    }
    print!("{}\n", ans)
}
