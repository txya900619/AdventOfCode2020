use std::io::{stdin, BufRead};

fn main() {
    let mut ans = 0;
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        if s == "stop" {
            break;
        }

        let mut row_l = 0;
        let mut row_r = 127;
        let mut col_l = 0;
        let mut col_r = 7;

        for c in s.trim().chars() {
            match c {
                'F' => row_r -= (row_r - row_l) / 2 + 1,
                'B' => row_l += (row_r - row_l) / 2 + 1,
                'R' => col_l += (col_r - col_l) / 2 + 1,
                'L' => col_r -= (col_r - col_l) / 2 + 1,
                _ => (),
            }
        }

        let v = row_l * 8 + col_l;

        if ans < v {
            ans = v;
        }
    }

    print!("{}\n", ans)
}
