use std::{
    io::{stdin, BufRead},
    vec,
};

fn main() {
    let mut init: Vec<Vec<bool>> = Vec::new(); //active is true
                                               // y,xn
    let mut cube: Vec<Vec<Vec<Vec<bool>>>> = Vec::new();
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s = s.trim();
        if s == "" {
            break;
        }
        init.push(
            s.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => false,
                })
                .collect(),
        )
    } //input

    let cube_length = init.len() + 6 * 2;
    let mut empty_two_dim: Vec<Vec<bool>> = Vec::new();
    empty_two_dim.resize(cube_length, vec![false; cube_length]);
    let empty_three_dim = vec![empty_two_dim.clone(); cube_length];

    let init: Vec<Vec<bool>> = empty_two_dim
        .iter()
        .enumerate()
        .map(|(y, vec)| {
            let init = init.clone();
            vec.iter()
                .enumerate()
                .map(move |(x, _)| {
                    if y > 5 && y < init.len() + 6 && x > 5 && x < init.len() + 6 {
                        init[y - 6][x - 6]
                    } else {
                        false
                    }
                })
                .collect()
        })
        .collect();
    // for vec in init.iter() {
    //     for v in vec {
    //         if *v {
    //             print!("#")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     print!("\n")
    // }
    let mut cube_3d: Vec<Vec<Vec<bool>>> = Vec::new();
    for i in 0..cube_length {
        if i == (cube_length) / 2 {
            cube_3d.push(init.clone());
            continue;
        }
        cube_3d.push(empty_two_dim.clone())
    }
    for i in 0..cube_length {
        if i == (cube_length) / 2 {
            cube.push(cube_3d.clone());
            continue;
        }
        cube.push(empty_three_dim.clone())
    }

    for _ in 0..6 {
        let cube_clone = cube.clone();
        for (w, three_dim) in cube_clone.iter().enumerate() {
            for (z, plat) in three_dim.iter().enumerate() {
                for (y, row) in plat.iter().enumerate() {
                    for (x, v) in row.iter().enumerate() {
                        let mut active_count = 0;
                        for w_add in -1..=1 {
                            for z_add in -1..=1 {
                                for y_add in -1..=1 {
                                    for x_add in -1..=1 {
                                        if w_add == 0 && z_add == 0 && y_add == 0 && x_add == 0 {
                                            continue;
                                        }
                                        let temp_w = w as i32 + w_add;
                                        let temp_z = z as i32 + z_add;
                                        let temp_y = y as i32 + y_add;
                                        let temp_x = x as i32 + x_add;

                                        if temp_w > -1 && temp_z > -1 && temp_y > -1 && temp_x > -1
                                        {
                                            if temp_w < cube_length as i32
                                                && temp_z < cube_length as i32
                                                && temp_y < cube_length as i32
                                                && temp_x < cube_length as i32
                                            {
                                                if cube_clone[temp_w as usize][temp_z as usize]
                                                    [temp_y as usize]
                                                    [temp_x as usize]
                                                {
                                                    active_count += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if *v {
                            if active_count != 3 && active_count != 2 {
                                cube[w][z][y][x] = false;
                            }
                        } else {
                            if active_count == 3 {
                                cube[w][z][y][x] = true;
                            }
                        }
                    }
                }
            }
        }
    }

    print!(
        "{}\n",
        cube.iter()
            .flat_map(|three_dim| three_dim.iter())
            .flat_map(|plat| plat.iter())
            .flat_map(|row| row.iter().filter(|&&v| v))
            .count()
    );
}
