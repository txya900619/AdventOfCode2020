use std::{
    io::{stdin, BufRead},
    vec,
};

fn main() {
    let mut group: Vec<char> = vec![];
    let mut ans = 0;
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s_trim = s.trim();

        if s_trim == "stop" {
            ans += group.len();
            group.clear();
            break;
        }

        if s_trim == "" {
            ans += group.len();
            group.clear();
            continue;
        }

        for c in s_trim.chars() {
            if !group.contains(&c) {
                group.push(c);
            }
        }
    }

    print!("{}\n", ans)
}
