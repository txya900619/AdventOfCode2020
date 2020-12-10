use std::io::{stdin, BufRead};

fn main() {
    let mut cid = false;
    let mut count = 0;
    let mut ans = 0;

    for s in stdin().lock().lines() {
        match s.unwrap().as_str() {
            "" => {
                if count == 8 || (count == 7 && !cid) {
                    ans += 1;
                }
                count = 0;
                cid = false;
            }
            "stop" => {
                if count == 8 || (count == 7 && !cid) {
                    ans += 1;
                }
                break;
            }
            s => {
                for field in s.split(" ") {
                    if let [field_name, _] = field.split(":").collect::<Vec<&str>>()[..] {
                        if field_name == "cid" {
                            cid = true
                        }
                        count += 1;
                    }
                }
            }
        }
    }

    print!("{}\n", ans);
}
