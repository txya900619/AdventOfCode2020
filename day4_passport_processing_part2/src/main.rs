use std::io::{stdin, BufRead};

fn main() {
    let mut cid = false;
    let mut count = 0;
    let mut invalid = false;
    let mut ans = 0;

    for s in stdin().lock().lines() {
        match s.unwrap().as_str() {
            "" => {
                if (count == 8 || (count == 7 && !cid)) && !invalid {
                    ans += 1;
                }
                print!("{}\n", ((count == 8 || (count == 7 && !cid)) && !invalid));
                count = 0;
                cid = false;
                invalid = false;
            }
            "stop" => {
                if (count == 8 || (count == 7 && !cid)) && !invalid {
                    ans += 1;
                }
                break;
            }
            s => {
                if invalid {
                    continue;
                }
                for field in s.split(" ") {
                    if invalid {
                        break;
                    }
                    if let [field_name, field_value] = field.split(":").collect::<Vec<&str>>()[..] {
                        match field_name {
                            "cid" => cid = true,
                            "byr" => {
                                let year = field_value.parse::<i32>().unwrap();
                                invalid = year > 2002 || year < 1920;
                            }
                            "iyr" => {
                                let year = field_value.parse::<i32>().unwrap();
                                invalid = year > 2020 || year < 2010;
                            }
                            "eyr" => {
                                let year = field_value.parse::<i32>().unwrap();
                                invalid = year > 2030 || year < 2020;
                            }
                            "hgt" => {
                                let mut value: Vec<char> = Vec::new();
                                let mut name: Vec<char> = Vec::new();
                                for c in field_value.chars() {
                                    if c.is_numeric() {
                                        value.push(c);
                                    } else {
                                        name.push(c)
                                    }
                                }
                                let value =
                                    value.iter().collect::<String>().parse::<i32>().unwrap();
                                match name.iter().collect::<String>().as_str() {
                                    "cm" => invalid = value > 193 || value < 150,
                                    "in" => invalid = value > 76 || value < 59,
                                    _ => invalid = true,
                                }
                            }
                            "hcl" => {
                                if field_value.len() != 7 {
                                    invalid = true;
                                    continue;
                                }
                                for (i, c) in field_value.chars().enumerate() {
                                    if i == 0 {
                                        if c == '#' {
                                            continue;
                                        } else {
                                            invalid = true;
                                            break;
                                        }
                                    }
                                    if (c < '0' || c > '9') && (c < 'a' || c > 'f') {
                                        invalid = true;
                                        break;
                                    }
                                }
                            }
                            "ecl" => {
                                invalid = field_value != "amb"
                                    && field_value != "blu"
                                    && field_value != "brn"
                                    && field_value != "gry"
                                    && field_value != "grn"
                                    && field_value != "hzl"
                                    && field_value != "oth"
                            }
                            "pid" => {
                                if field_value.len() != 9 {
                                    invalid = true;
                                    continue;
                                }
                                for c in field_value.chars() {
                                    if !c.is_numeric() {
                                        invalid = true;
                                        break;
                                    }
                                }
                            }
                            _ => (),
                        }
                        count += 1;
                    }
                }
            }
        }
    }

    print!("{}\n", ans);
}
