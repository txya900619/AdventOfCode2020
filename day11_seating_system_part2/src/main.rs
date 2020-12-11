use std::io::{stdin, BufRead};

fn main() {
    let mut sits: Vec<Vec<char>> = Vec::new();
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s_trim = s.trim();

        if s_trim == "stop" {
            break;
        }

        let mut row: Vec<char> = Vec::new();

        for c in s_trim.chars() {
            if c == 'L' {
                row.push('#')
            } else {
                row.push('.')
            }
        }

        sits.push(row);
    }

    loop {
        let next = next_round(&sits);
        let mut same = true;
        'outer: for i in 0..sits.len() {
            for j in 0..sits[0].len() {
                if sits[i][j] != next[i][j] {
                    same = false;
                    break 'outer;
                }
            }
        }
        sits = next;
        if same {
            break;
        }
    }
    print!(
        "{}\n",
        sits.iter()
            .map(|row| row
                .iter()
                .map(|&c| if c == '#' { 1 } else { 0 })
                .sum::<i32>())
            .sum::<i32>()
    )
}
fn next_round(sits: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    sits.iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_index, &c)| {
                    let mut count = 0;
                    for i in -1..=1 {
                        for j in -1..=1 {
                            if i == 0 && j == 0 {
                                continue;
                            }
                            let mut row_index = row_index as i32 + i;
                            let mut col_index = col_index as i32 + j;
                            while row_index >= 0
                                && row_index < sits.len() as i32
                                && col_index >= 0
                                && col_index < row.len() as i32
                            {
                                let sit = sits[row_index as usize][col_index as usize];
                                if sit != '.' {
                                    if sit == '#' {
                                        count += 1;
                                    }
                                    break;
                                }
                                row_index += i;
                                col_index += j;
                            }
                        }
                    }
                    match c {
                        '#' => {
                            if count >= 5 {
                                'L'
                            } else {
                                '#'
                            }
                        }
                        'L' => {
                            if count == 0 {
                                '#'
                            } else {
                                'L'
                            }
                        }
                        c => c,
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}
