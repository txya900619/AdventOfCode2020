use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let mut fields: Vec<Field> = Vec::new();
    let mut tickets: Vec<Vec<i32>> = Vec::new();
    let mut sorted_fields: HashMap<usize, Field> = HashMap::new();
    let mut used_fields: HashSet<Field> = HashSet::new();
    for _ in 0..20 {
        let s = lines.next().unwrap().unwrap();
        let name_and_ranges_str = s.split(": ").collect::<Vec<&str>>();
        let ranges_vec: Vec<(i32, i32)> = name_and_ranges_str[1]
            .split(" or ")
            .map(|s| {
                let range = s
                    .split("-")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<i32>>();
                return (range[0], range[1]);
            })
            .collect();
        fields.push(Field {
            name: String::from(name_and_ranges_str[0]),
            range_one: ranges_vec[0],
            range_two: ranges_vec[1],
        })
    }
    for s in lines {
        let s = s.unwrap();
        let s = s.trim();
        if s == "stop" {
            break;
        }
        if s == "your ticket:" || s == "nearby tickets:" || s == "" {
            continue;
        }
        tickets.push(s.split(",").map(|s| s.parse().unwrap()).collect())
    }

    let valid_tickets: Vec<&Vec<i32>> = tickets
        .iter()
        .filter(|vec| vec.iter().all(|&n| fields.iter().any(|f| f.is_valid(n))))
        .collect();

    while sorted_fields.len() < tickets[0].len() {
        for field in fields.iter() {
            if used_fields.contains(&field) {
                continue;
            }
            let mut matched = 0;
            let mut matched_index = 0;
            for i in 0..tickets[0].len() {
                if sorted_fields.contains_key(&i) {
                    continue;
                }
                if valid_tickets.iter().all(|t| field.is_valid(t[i])) {
                    if matched >= 1 {
                        matched += 1;
                        break;
                    }
                    matched += 1;
                    matched_index = i;
                }
            }
            if matched == 1 {
                sorted_fields.insert(matched_index, field.clone());
                used_fields.insert(field.clone());
            }
        }
    }

    let ans: i64 = tickets[0]
        .iter()
        .enumerate()
        .filter(|(i, _)| sorted_fields[i].name.contains("departure"))
        .map(|(_, &n)| n as i64)
        .product();

    print!("{}\n", ans);
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Field {
    name: String,
    range_one: (i32, i32),
    range_two: (i32, i32),
}

impl Field {
    fn is_valid(&self, n: i32) -> bool {
        (n >= self.range_one.0 && n <= self.range_one.1)
            || (n >= self.range_two.0 && n <= self.range_two.1)
    }
}
