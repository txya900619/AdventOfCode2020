use std::io::{stdin, BufRead};

fn main() {
    let mut numbers: Vec<u64> = Vec::new();
    let mut ans: u64 = 0;
    let mut ans_index: usize = 0;
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s_trim = s.trim();
        if s_trim == "stop" {
            break;
        }
        numbers.push(s_trim.parse::<u64>().unwrap())
    }
    'outer: for (i, num) in numbers.iter().enumerate() {
        if i < 25 {
            continue;
        }
        for (first_index, first) in numbers[i - 25..i].iter().enumerate() {
            for second in numbers[first_index + 1..i].iter() {
                if *num == first + second {
                    continue 'outer;
                }
            }
        }
        ans = *num;
        ans_index = i;
        break;
    }
    for (i, first) in numbers[..ans_index].iter().enumerate() {
        let mut temp_num = *first;
        let mut min = temp_num;
        let mut max: u64 = 0;
        for &after in numbers[i + 1..ans_index].iter() {
            if temp_num > ans {
                break;
            }
            temp_num += after;
            if min > after {
                min = after;
            }
            if max < after {
                max = after;
            }
            if temp_num == ans {
                print!("{}\n", min + max);
                return;
            }
        }
    }
}
