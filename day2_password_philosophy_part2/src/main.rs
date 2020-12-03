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
            if let [indexes, keyword] = policy.split(" ").collect::<Vec<&str>>()[..] {
                let keyword = keyword.parse::<char>().unwrap();
                let indexes = indexes
                    .split("-")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                let chars: Vec<char> = password.trim().chars().collect();
                let mut count = 0;
                for index in indexes {
                    if chars[index - 1] == keyword {
                        count += 1;
                    }
                }
                if count == 1 {
                    ans += 1;
                }
            }
        }
    }
    print!("{}\n", ans)
}
