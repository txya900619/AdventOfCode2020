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
                .map(move |(col_index, c)| {
                    let row_index_l = row_index.saturating_sub(1);
                    let row_index_r = if row_index + 1 >= sits.len() {
                        row_index
                    } else {
                        row_index + 1
                    };

                    let col_index_l = col_index.saturating_sub(1);
                    let col_index_r = if col_index + 1 >= row.len() {
                        col_index
                    } else {
                        col_index + 1
                    };

                    match c {
                        'L' => {
                            for i in row_index_l..=row_index_r {
                                for j in col_index_l..=col_index_r {
                                    if i == row_index && j == col_index {
                                        continue;
                                    }
                                    if sits[i][j] == '#' {
                                        return 'L';
                                    }
                                }
                            }
                            '#'
                        }
                        '#' => {
                            let mut count = 0;

                            for i in row_index_l..=row_index_r {
                                for j in col_index_l..=col_index_r {
                                    if i == row_index && j == col_index {
                                        continue;
                                    }
                                    if sits[i][j] == '#' {
                                        count += 1;
                                    }
                                }
                            }

                            if count >= 4 {
                                'L'
                            } else {
                                '#'
                            }
                        }
                        &c => c,
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}
