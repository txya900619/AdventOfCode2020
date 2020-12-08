use std::{
    collections::HashMap,
    io::{stdin, BufRead},
    vec,
};

fn main() {
    let mut bag_map: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    for s in stdin().lock().lines() {
        let s = s.unwrap();
        let s_trim = s.trim();

        if s_trim == "stop" {
            break;
        }

        if let [bag_name, bag_contains] = s_trim.split(" contain ").collect::<Vec<&str>>()[..] {
            let mut contains: Vec<(i32, String)> = vec![];
            if bag_contains != "no other bags." {
                for bag_contain in bag_contains.split(", ") {
                    contains.push((
                        bag_contain[..1].parse::<i32>().unwrap(),
                        bag_contain
                            .trim()
                            .replace(".", "")
                            .replace(" bags", "")
                            .replace(" bag", "")[2..]
                            .to_string(),
                    ))
                }
            }
            bag_map.insert(bag_name.replace(" bags", ""), contains);
        }
    }

    print!(
        "{}\n",
        calc_bag_contains(&"shiny gold".to_string(), &bag_map)
    );
}

fn calc_bag_contains(bag_name: &String, bag_map: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    match bag_map.get(bag_name) {
        Some(bag_contains) => {
            let mut amont_of_contain = 0;
            for bag_contain in bag_contains {
                amont_of_contain +=
                    bag_contain.0 + bag_contain.0 * calc_bag_contains(&bag_contain.1, bag_map);
            }
            amont_of_contain
        }
        None => 0,
    }
}
