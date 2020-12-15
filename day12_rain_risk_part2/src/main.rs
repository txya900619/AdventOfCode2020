use std::io::{stdin, BufRead};

fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        if s.trim() == "stop" {
            break;
        }
        let mut s_trim: Vec<char> = s.trim().chars().collect();
        let command: char = s_trim.remove(0);
        let value = s_trim.iter().collect::<String>().parse::<i32>().unwrap();
        match command {
            'N' => waypoint_y += value,
            'S' => waypoint_y -= value,
            'E' => waypoint_x += value,
            'W' => waypoint_x -= value,
            'L' => {
                for _ in 0..(value / 90) {
                    let temp = waypoint_y;
                    waypoint_y = waypoint_x;
                    waypoint_x = -temp;
                }
            }

            'R' => {
                for _ in 0..(value / 90) {
                    let temp = waypoint_x;
                    waypoint_x = waypoint_y;
                    waypoint_y = -temp;
                }
            }
            'F' => {
                for _ in 0..value {
                    x += waypoint_x;
                    y += waypoint_y;
                }
            }
            _ => (),
        }
    }
    print!("{}\n", x.abs() + y.abs());
}
