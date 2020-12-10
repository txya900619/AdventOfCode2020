use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    vec,
};

fn main() {
    let mut ans = 0;
    let mut bag_map: HashMap<String, Vec<String>> = HashMap::new();
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s_trim = s.trim();

        if s_trim == "stop" {
            break;
        }

        if let [bag_name, bag_contains] = s_trim.split(" contain ").collect::<Vec<&str>>()[..] {
            let mut contains: Vec<String> = vec![];
            if bag_contains != "no other bags." {
                for bag_contain in bag_contains.split(", ") {
                    contains.push(
                        bag_contain
                            .trim()
                            .replace(".", "")
                            .replace(" bags", "")
                            .replace(" bag", "")[2..]
                            .to_string(),
                    )
                }
            }
            bag_map.insert(bag_name.replace(" bags", ""), contains);
        }
    }

    for (bag_name, _) in bag_map.iter() {
        if contain_shiny_gold(bag_name, &bag_map) {
            ans += 1;
        }
    }
    print!("{}\n", ans);
}

fn contain_shiny_gold(bag_name: &String, bag_map: &HashMap<String, Vec<String>>) -> bool {
    match bag_map.get(bag_name) {
        Some(bag_contains) => {
            for bag_contain in bag_contains {
                if bag_contain == "shiny gold" {
                    return true;
                }
                if contain_shiny_gold(bag_contain, bag_map) {
                    return true;
                }
            }
            false
        }
        None => false,
    }
}
