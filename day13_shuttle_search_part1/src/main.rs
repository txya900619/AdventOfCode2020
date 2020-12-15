use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut iterator = stdin.lock().lines();
    let line1 = iterator.next().unwrap().unwrap();
    let line2 = iterator.next().unwrap().unwrap();
    let earliest: i32 = line1.parse().unwrap();
    let mut ans = earliest * 2;
    let mut ans_bus_id = 0;
    for c in line2.split(",") {
        if c == "x" {
            continue;
        }
        let bus_id: i32 = c.parse().unwrap();

        if earliest % bus_id == 0 {
            ans = earliest;
            ans_bus_id = bus_id;
            break;
        }
        let temp = ((earliest / bus_id) + 1) * bus_id;
        if temp < ans {
            ans = temp;
            ans_bus_id = bus_id;
        }
    }

    print!("{}\n", (ans - earliest) * ans_bus_id);
}
