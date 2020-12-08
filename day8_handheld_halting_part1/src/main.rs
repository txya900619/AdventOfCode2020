use std::{
    convert::TryFrom,
    io::{stdin, BufRead},
};

fn main() {
    let mut instructions: Vec<(String, i32)> = Vec::new();
    let mut accumulator: i32 = 0;
    let mut current_index: usize = 0;
    let mut runed_index: Vec<usize> = Vec::new();
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

    loop {
        if runed_index.contains(&current_index) {
            break;
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

    print!("{}\n", accumulator);
}
