use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let mut ranges: Vec<(Range, Range)> = Vec::new();

    let mut ans = 0;
    for _ in 0..20 {
        let s = lines.next().unwrap().unwrap();
        let ranges_vec: Vec<Range> = s.split(": ").collect::<Vec<&str>>()[1]
            .split(" or ")
            .map(|s| {
                let s_split = s
                    .split("-")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<i32>>();
                return Range {
                    min: s_split[0],
                    max: s_split[1],
                };
            })
            .collect();
        ranges.push((ranges_vec[0], ranges_vec[1]))
    }
    for s in lines {
        let s = s.unwrap();
        let s = s.trim();
        if s == "stop" {
            break;
        }
        if s == "your ticket:" || s == "nearby tickets:" || s == "" {
            continue;
        }
        for n in s.split(",") {
            let n = n.parse::<i32>().unwrap();
            let mut invalid = true;
            for r in ranges.iter() {
                let range_one = r.0;
                let range_two = r.1;
                if (n >= range_one.min && n <= range_one.max)
                    || (n >= range_two.min && n <= range_two.max)
                {
                    invalid = false;
                }
            }
            if invalid {
                ans += n;
            }
        }
    }
    print!("{}\n", ans);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Range {
    min: i32,
    max: i32,
}
