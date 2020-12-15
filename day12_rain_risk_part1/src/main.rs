use std::io::{stdin, BufRead};

fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut direction = 0; // 0 is E, 1 is N, 2 is W, 3 is S
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        if s.trim() == "stop" {
            break;
        }
        let mut s_trim: Vec<char> = s.trim().chars().collect();
        let command: char = s_trim.remove(0);
        let value = s_trim.iter().collect::<String>().parse::<i32>().unwrap();
        match command {
            'N' => y += value,
            'S' => y -= value,
            'E' => x += value,
            'W' => x -= value,
            'L' => direction = (4 + direction + value / 90) % 4,
            'R' => direction = (4 + direction - value / 90) % 4,
            'F' => match direction {
                0 => x += value,
                1 => y += value,
                2 => x -= value,
                3 => y -= value,
                _ => (),
            },
            _ => (),
        }
    }
    print!("{}\n", x.abs() + y.abs());
}
