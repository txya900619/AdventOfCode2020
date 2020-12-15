use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();
    let mut count = 0;
    let mut bus: Vec<(u64, u64)> = Vec::new();
    for c in lines.next().unwrap().unwrap().split(",") {
        if c != "x" {
            bus.push((c.parse().unwrap(), count));
        }
        count += 1;
    }

    let (top, _) = bus.remove(0);
    let mut ans = 0;
    let mut step_size = top;
    for (bus_id, bus_after) in bus {
        while (ans + bus_after) % bus_id != 0 {
            ans += step_size;
        }
        step_size *= bus_id;
    }

    print!("{}\n", ans);
}
//TODO: research how to solve
