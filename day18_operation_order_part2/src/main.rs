use std::io::{stdin, BufRead};

fn main() {
    let mut ans: i64 = 0;
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s = s.trim().replace(" ", "");
        if s == "stop" {
            break;
        }
        ans += solve(s.as_str()) as i64;
    }
    print!("{}\n", ans);
}

fn solve(input: &str) -> u64 {
    let mut result = 0;
    let mut parentheses_count = 0;
    let mut temp_input = String::new();
    let mut operator = '+';
    for (i, c) in input.chars().enumerate() {
        if parentheses_count != 0 {
            if c == ')' {
                parentheses_count -= 1
            };
            if c == '(' {
                parentheses_count += 1;
            }
            if parentheses_count == 0 {
                result = calc(result, solve(temp_input.as_str()), operator);
                temp_input.clear();
                continue;
            }
            temp_input.push(c);
        } else {
            match c {
                '+' => operator = '+',
                '*' => {
                    result = calc(result, solve(&input[i + 1..]), '*');
                    break;
                }
                '(' => parentheses_count += 1,
                n => result = calc(result, n.to_digit(10).unwrap() as u64, operator),
            }
        }
    }
    result
}

fn calc(l: u64, r: u64, op: char) -> u64 {
    match op {
        '*' => l * r,
        '+' => l + r,
        _ => 0,
    }
}
