use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    vec,
};

type RuleID = usize;
enum Rule {
    Char(char),
    OneRule(Vec<RuleID>),
    TwoRule(Vec<RuleID>, Vec<RuleID>),
}

struct Rules {
    rules: HashMap<RuleID, Rule>,
}

impl Rules {
    fn new() -> Self {
        Rules {
            rules: HashMap::new(),
        }
    }

    fn parse_one_rule(&mut self, input: &str) {
        if let [rule_id, rule_str] = input.split(": ").collect::<Vec<&str>>()[..] {
            let rule: Rule;
            let rule_id: RuleID = rule_id.parse().unwrap();
            if let [rule_one, rule_two] = rule_str.split(" | ").collect::<Vec<&str>>()[..] {
                let rule_one: Vec<RuleID> =
                    rule_one.split(" ").map(|s| s.parse().unwrap()).collect();
                let rule_two: Vec<RuleID> =
                    rule_two.split(" ").map(|s| s.parse().unwrap()).collect();
                rule = Rule::TwoRule(rule_one, rule_two);
            } else if rule_str.starts_with("\"") && rule_str.ends_with("\"") {
                rule = Rule::Char(rule_str.chars().nth(1).unwrap());
            } else {
                rule = Rule::OneRule(rule_str.split(" ").map(|s| s.parse().unwrap()).collect());
            }
            self.rules.insert(rule_id, rule);
        }
    }

    fn part2_change_rule(&mut self) {
        self.rules.insert(8, Rule::TwoRule(vec![42, 8], vec![42]));
        self.rules
            .insert(11, Rule::TwoRule(vec![42, 31], vec![42, 11, 31]));
    }

    fn match_seq_tail<'a>(&self, seq: &[RuleID], input: &'a str) -> Vec<&'a str> {
        let mut result = self.match_seq_non_tail(seq, input);
        let mut reminds = &result[..];
        while !reminds.is_empty() {
            let mut new_result = reminds
                .iter()
                .flat_map(|r| self.match_seq_non_tail(seq, r))
                .collect::<Vec<&str>>();
            let from = result.len();
            result.append(&mut new_result);
            reminds = &result[from..];
        }

        result
    }

    fn match_seq_non_tail<'a>(&self, seq: &[RuleID], input: &'a str) -> Vec<&'a str> {
        seq.iter().fold(vec![input], |remains, rule| {
            remains
                .iter()
                .flat_map(|remain| self.match_rule(rule, remain))
                .collect()
        })
    }

    fn match_seq<'a>(&self, rule_id: &RuleID, seq: &[RuleID], input: &'a str) -> Vec<&'a str> {
        if seq.last() == Some(rule_id) {
            self.match_seq_tail(&seq[..seq.len() - 1], input)
        } else {
            self.match_seq_non_tail(seq, input)
        }
    }

    fn match_rule<'a>(&self, rule_id: &RuleID, input: &'a str) -> Vec<&'a str> {
        match self.rules.get(rule_id).unwrap() {
            Rule::Char(c) => {
                if input.chars().next() == Some(*c) {
                    vec![&input[c.len_utf8()..]]
                } else {
                    vec![]
                }
            }
            Rule::OneRule(rule_id_seq) => self.match_seq(rule_id, rule_id_seq, input),
            Rule::TwoRule(rule_id_seq_one, rule_id_seq_two) => {
                let result_one = self.match_seq(rule_id, rule_id_seq_one, input);
                if !result_one.is_empty() {
                    result_one
                } else {
                    self.match_seq(rule_id, rule_id_seq_two, input)
                }
            }
        }
    }

    fn match_zero<'a>(&self, input: &'a str) -> Result<(), &'a str> {
        let result = self.match_rule(&0, input);
        match result.iter().next() {
            None => Err("no match"),
            Some(s) if s.is_empty() => Ok(()),
            _ => Err("unexpected result"),
        }
    }
}

fn main() {
    let mut rules = Rules::new();
    let mut ans = 0;
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s = s.trim();
        if s == "" {
            break;
        }

        rules.parse_one_rule(s);
    }

    rules.part2_change_rule();

    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s = s.trim();
        if s == "stop" {
            break;
        }

        match rules.match_zero(s) {
            Ok(_) => ans += 1,
            _ => (),
        }
    }

    print!("{}\n", ans);
}
