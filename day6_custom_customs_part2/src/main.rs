use std::{
    io::{stdin, BufRead},
    vec,
};

fn main() {
    let mut group: Vec<char> = vec![];
    let mut ans = 0;
    let mut first = true;
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
            first = true;
            continue;
        }

        if first {
            for c in s_trim.chars() {
                group.push(c);
            }
            first = false;
        } else {
            group = group.into_iter().filter(|&c| s_trim.contains(c)).collect()
        }
    }

    print!("{}\n", ans)
}
