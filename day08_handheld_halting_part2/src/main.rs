use std::{
    convert::TryFrom,
    io::{stdin, BufRead},
};

fn main() {
    let mut instructions: Vec<(String, i32)> = Vec::new();
    let mut ans: i32 = 0;
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s_trim = s.trim();
        if s_trim == "stop" {
            break;
        }
        if let [operator, value] = s_trim.split(" ").collect::<Vec<&str>>()[..] {
            instructions.push((operator.to_owned(), value.parse::<i32>().unwrap()));
        }
    }

    for (index, (operator, _)) in instructions.iter().enumerate() {
        let mut temp_instructions = instructions.clone();
        match operator.as_str() {
            "jmp" => temp_instructions[index].0 = "nop".to_owned(),
            "nop" => temp_instructions[index].0 = "jmp".to_owned(),
            _ => continue,
        }
        match run_instructions(temp_instructions) {
            Ok(accumulator) => {
                ans = accumulator;
                break;
            }
            Err(_) => {
                continue;
            }
        }
    }

    print!("{}\n", ans);
}

fn run_instructions(instructions: Vec<(String, i32)>) -> Result<i32, ()> {
    let mut current_index: usize = 0;
    let mut runed_index: Vec<usize> = Vec::new();
    let mut accumulator: i32 = 0;

    loop {
        if runed_index.contains(&current_index) {
            return Err(());
        }
        if current_index >= instructions.len() {
            return Ok(accumulator);
        }
        runed_index.push(current_index);
        let (operator, value) = &instructions[current_index];
        match operator.as_str() {
            "acc" => accumulator += value,
            "jmp" => {
                if *value > 0 {
                    current_index += usize::try_from(value.clone()).unwrap();
                } else {
                    current_index -= usize::try_from(value.clone().abs()).unwrap();
                }

                continue;
            }
            "nop" => (),
            _ => (),
        }
        current_index += 1
    }
}
