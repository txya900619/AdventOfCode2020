use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let mut mask = String::new();
    let mut ans: HashMap<i32, i64> = HashMap::new();
    for s in stdin().lock().lines() {
        let s = s.unwrap().trim().to_owned();
        if s == "stop" {
            break;
        }
        if s.contains("mask") {
            mask = s.split(" ").collect::<Vec<&str>>()[2].to_owned();
            continue;
        }

        let inputs = s.split(" ").collect::<Vec<&str>>();

        let value = format!("{:036b}", inputs[2].parse::<i64>().unwrap());

        let value = i64::from_str_radix(
            &value
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    let m = mask.chars().nth(i).unwrap();
                    if m == 'X' || m == c {
                        c
                    } else {
                        m
                    }
                })
                .collect::<String>(),
            2,
        )
        .unwrap();

        ans.insert(
            inputs[0][inputs[0].find('[').unwrap() + 1..inputs[0].find(']').unwrap()]
                .parse::<i32>()
                .unwrap(),
            value,
        );
    }

    print!("{}\n", ans.values().sum::<i64>())
}
