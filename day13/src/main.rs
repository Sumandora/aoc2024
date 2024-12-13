use std::io::{self, Read};

use regex::Regex;

fn numbers(str: &str) -> Vec<i64> {
    let re = Regex::new(r"([-+]?\d+)").unwrap();
    let mut vec = Vec::new();
    for (_, [num]) in re.captures_iter(str).map(|cap| cap.extract()) {
        vec.push(num.parse::<i64>().unwrap());
    }
    vec
}

#[derive(Debug, Clone)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn solve(machine: &Machine) -> (f64, f64) {
    // solve for  c_0 and c_1 in a_0*c_0=p_0-b_0*c_1 and a_1*c_0=p_1-b_1*c_1

    // c_0 = (b_1 p_0 - b_0 p_1)/(a_0 b_1 - a_1 b_0)
    // c_1 = (a_1 p_0 - a_0 p_1)/(a_1 b_0 - a_0 b_1)
    // and a_1 b_0!=a_0 b_1 and b_1 !=0 (this line doesn't really matter but kept for completeness)

    let a_0 = machine.button_a.0;
    let a_1 = machine.button_a.1;
    let b_0 = machine.button_b.0;
    let b_1 = machine.button_b.1;
    let p_0 = machine.prize.0;
    let p_1 = machine.prize.1;

    let c_0 = (b_1 * p_0 - b_0 * p_1) as f64 / (a_0 * b_1 - a_1 * b_0) as f64;
    let c_1 = (a_1 * p_0 - a_0 * p_1) as f64 / (a_1 * b_0 - a_0 * b_1) as f64;

    (c_0, c_1)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let machines = input
        .split("\n\n")
        .map(|str| {
            let mut lines = str.lines();

            let button_a = numbers(lines.next().unwrap());
            let button_b = numbers(lines.next().unwrap());
            let prize = numbers(lines.next().unwrap());

            Machine {
                button_a: (*button_a.get(0).unwrap(), *button_a.get(1).unwrap()),
                button_b: (*button_b.get(0).unwrap(), *button_b.get(1).unwrap()),
                prize: (*prize.get(0).unwrap(), *prize.get(1).unwrap()),
            }
        })
        .collect::<Vec<_>>();

    let part1 = machines
        .iter()
        .map(|machine| {
            let (c_0, c_1) = solve(&machine);

            if c_0 % 1.0 != 0.0 || c_1 % 1.0 != 0.0 {
                return 0;
            }

            let c_0 = c_0 as u64;
            let c_1 = c_1 as u64;

            if c_0 >= 100 || c_1 >= 100 {
                return 0;
            }

            return c_0 * 3 + c_1 * 1;
        })
        .sum::<u64>();

    let part2 = machines
        .iter()
        .map(|machine| {
            let mut new_machine = machine.clone();
            new_machine.prize.0 += 10000000000000;
            new_machine.prize.1 += 10000000000000;
            let (c_0, c_1) = solve(&new_machine);

            if c_0 % 1.0 != 0.0 || c_1 % 1.0 != 0.0 {
                return 0;
            }

            let c_0 = c_0 as u64;
            let c_1 = c_1 as u64;

            return c_0 * 3 + c_1 * 1;
        })
        .sum::<u64>();

    println!("Part 1: {part1}\nPart 2: {part2}");
}
