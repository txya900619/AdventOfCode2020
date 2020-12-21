use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let mut nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    for _ in nums.len() as i32 - 1..2019 {
        let last = nums.last().unwrap();
        if nums.iter().filter(|&v| last == v).count() > 1 {
            let mut first_index = -1;
            let mut second_index = -1;
            for (i, rev_num) in nums.iter().enumerate().rev() {
                if last == rev_num {
                    if first_index > -1 {
                        second_index = i as i32;
                        break;
                    } else {
                        first_index = i as i32;
                    }
                }
            }
            nums.push(first_index - second_index);
        } else {
            nums.push(0);
        }
    }
    print!("{}\n", nums.last().unwrap());
}
