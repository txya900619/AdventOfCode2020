use std::io::{stdin, BufRead};

fn main() {
    let mut nums: Vec<u32> = vec![0];
    let mut ans: u64 = 1;

    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s_trim = s.trim();
        if s_trim == "stop" {
            break;
        }
        nums.push(s_trim.parse().unwrap())
    }
    nums.sort();
    nums.push(nums[nums.len() - 1] + 3);

    let mut continuous_count = 0;
    for (index, num) in nums.iter().enumerate() {
        if index > 0 && index < nums.len() - 1 {
            if nums[index + 1] - num < 3 && num - nums[index - 1] < 3 {
                continuous_count += 1;
            } else {
                match continuous_count {
                    0 => (),
                    1 => ans *= 2,
                    2 => ans *= 4,
                    3 => ans *= 7,
                    _ => (),
                }
                continuous_count = 0;
            }
        }
    }
    print!("{}\n", ans);
}
