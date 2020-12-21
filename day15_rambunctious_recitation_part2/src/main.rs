use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let input: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();

    let mut last = *input.last().unwrap();

    let mut num_map: HashMap<usize, usize> = input
        .iter()
        .take(input.len() - 1)
        .enumerate()
        .map(|(i, &v)| (v, i + 1))
        .collect();

    for i in input.len() + 1..30000001 {
        let new = match num_map.get(&last) {
            Some(last_i) => i - 1 - last_i,
            None => 0,
        };
        num_map.insert(last, i - 1);
        last = new as usize;
    }
    print!("{}\n", last);
}
