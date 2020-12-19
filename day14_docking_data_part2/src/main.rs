use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    vec,
};

fn main() {
    let mut mask = String::new();
    let mut ans: HashMap<i64, i64> = HashMap::new();
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
        let value = inputs[2].parse::<i64>().unwrap();

        let mut origin_mem = format!(
            "{:036b}",
            inputs[0][inputs[0].find('[').unwrap() + 1..inputs[0].find(']').unwrap()]
                .parse::<i32>()
                .unwrap()
        );

        origin_mem = origin_mem
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let m = mask.chars().nth(i).unwrap();
                if m == 'X' {
                    'X'
                } else if m == '1' {
                    '1'
                } else {
                    c
                }
            })
            .collect();

        for mem in find_all_mem(origin_mem) {
            ans.insert(mem, value);
        }
    }

    print!("{}\n", ans.values().sum::<i64>())
}

fn find_all_mem(raw: String) -> Vec<i64> {
    if let Some(i) = raw.find('X') {
        let raw_one = raw[0..i].to_owned() + "0" + &raw[i + 1..];
        let raw_two = raw[0..i].to_owned() + "1" + &raw[i + 1..];
        let mut ans = find_all_mem(raw_one);
        ans.append(&mut find_all_mem(raw_two));
        ans
    } else {
        vec![i64::from_str_radix(raw.as_str(), 2).unwrap()]
    }
}
