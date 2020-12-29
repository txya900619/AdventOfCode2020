use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let mut solve = Solve::new();
    let mut ans = 0;
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s = s.trim();
        if s == "" {
            break;
        }

        let mut flag = false;
        let mut complete_v = String::new();
        let rule = s.split(": ").collect::<Vec<&str>>();
        solve.rules.insert(
            rule[0].to_string(),
            rule[1]
                .split(" | ")
                .map(|s| {
                    if let Some(c) = s.chars().nth(1) {
                        if c.is_alphabetic() {
                            flag = true;
                            complete_v = c.to_string();
                            return c.to_string();
                        }
                    }

                    s.to_string()
                })
                .collect(),
        );
        if flag {
            solve
                .complete_rules
                .insert(rule[0].to_string(), vec![complete_v]);
        }
    }

    print!("{}\n", solve.parse(&"0".to_string()).len());

    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s = s.trim();
        if s == "stop" {
            break;
        }
        if solve
            .complete_rules
            .get(&"0".to_owned())
            .unwrap()
            .contains(&s.to_owned())
        {
            ans += 1;
        }
    }

    print!("{}\n", ans);
}

struct Solve {
    rules: HashMap<String, Vec<String>>,
    complete_rules: HashMap<String, Vec<String>>,
}

impl Solve {
    fn new() -> Solve {
        Self {
            rules: HashMap::new(),
            complete_rules: HashMap::new(),
        }
    }

    fn parse<'a>(&'a mut self, k: &String) -> Vec<String> {
        if let Some(v) = self.complete_rules.get(k) {
            return v.clone();
        }

        let v = self.rules.get(k).unwrap().clone();
        let mut complete_v: Vec<String> = Vec::new();

        for s in v.iter() {
            for s_or in s.split(" | ") {
                let mut parsed: Vec<Vec<String>> = Vec::new();
                for s_split in s_or.split(" ") {
                    parsed.push(self.parse(&s_split.to_string()))
                }

                if parsed.len() > 1 {
                    for first in &parsed[0] {
                        for sec in &parsed[1] {
                            let result = first.clone() + sec;
                            complete_v.push(result);
                        }
                    }
                } else {
                    for first in &parsed[0] {
                        complete_v.push(first.clone());
                    }
                }
            }
        }

        self.complete_rules.insert(k.clone(), complete_v.clone());

        complete_v
    }
}
