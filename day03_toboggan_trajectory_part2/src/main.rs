use std::io::{stdin, BufRead};

fn main() {
    let mut map: Vec<String> = Vec::new();
    let slops = Vec::from([(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);
    let mut ans = 0;
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        if s == "stop" {
            break;
        }
        map.push(s);
    }

    for slop in slops {
        let mut col = slop.0;
        let mut row = slop.1;
        let mut count = 0;
        while row < map.len() {
            if map[row].chars().collect::<Vec<char>>()[col] == '#' {
                count += 1;
            }
            col = (col + slop.0) % map[row].len();
            row += slop.1;
        }
        if ans == 0 {
            ans = count;
        } else {
            ans *= count;
        }
    }

    print!("{}\n", ans);
}
